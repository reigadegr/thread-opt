use std::{
    cmp,
    collections::{hash_map::Entry, HashMap},
    fs,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver, Sender, SyncSender},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};

use anyhow::Result;
use libc::{pid_t, sysconf, _SC_CLK_TCK};
#[cfg(debug_assertions)]
use log::debug;

#[derive(Debug, Clone)]
struct UsageTracker {
    pid: pid_t,
    tid: pid_t,
    last_cputime: u64,
    read_timer: Instant,
}

impl UsageTracker {
    fn new(pid: pid_t, tid: pid_t) -> Result<Self> {
        Ok(Self {
            pid,
            tid,
            last_cputime: get_thread_cpu_time(pid, tid)?,
            read_timer: Instant::now(),
        })
    }

    fn try_calculate(&mut self) -> Result<f64> {
        let tick_per_sec = unsafe { sysconf(_SC_CLK_TCK) };
        let new_cputime = get_thread_cpu_time(self.pid, self.tid)?;
        let elapsed_ticks = self.read_timer.elapsed().as_secs_f64() * tick_per_sec as f64;
        self.read_timer = Instant::now();
        let cputime_slice = new_cputime - self.last_cputime;
        self.last_cputime = new_cputime;
        Ok(cputime_slice as f64 / elapsed_ticks)
    }
}

#[derive(Debug)]
pub struct ProcessMonitor {
    stop: Arc<AtomicBool>,
    sender: SyncSender<Option<pid_t>>,
    max_usage_tid: Receiver<pid_t>,
}

impl ProcessMonitor {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::sync_channel(0);
        let stop = Arc::new(AtomicBool::new(false));
        let (max_usage_tid_sender, max_usage_tid) = mpsc::channel();

        {
            let stop = stop.clone();

            thread::Builder::new()
                .name("ProcessMonitor".to_string())
                .spawn(move || {
                    monitor_thread(&stop, &receiver, &max_usage_tid_sender);
                })
                .unwrap();
        }

        Self {
            stop,
            sender,
            max_usage_tid,
        }
    }

    pub fn set_pid(&self, pid: Option<pid_t>) {
        self.sender.send(pid).unwrap();
    }

    fn stop(&self) {
        self.stop.store(true, Ordering::Release);
    }

    pub fn update_max_usage_tid(&self) -> Option<pid_t> {
        self.max_usage_tid.try_iter().last()
    }
}

impl Drop for ProcessMonitor {
    fn drop(&mut self) {
        self.stop();
    }
}

fn monitor_thread(
    stop: &Arc<AtomicBool>,
    receiver: &Receiver<Option<pid_t>>,
    max_usage_tid: &Sender<pid_t>,
) {
    let mut current_pid = None;
    let mut last_full_update = Instant::now();
    let mut all_trackers = HashMap::new();
    let mut top_trackers = HashMap::new();

    while !stop.load(Ordering::Acquire) {
        if let Ok(pid) = receiver.try_recv() {
            current_pid = pid;
            all_trackers.clear();
            top_trackers.clear();
        }

        if let Some(pid) = current_pid {
            if last_full_update.elapsed() > Duration::from_millis(2500) {
                #[cfg(debug_assertions)]
                debug!("开始计算喵");
                let Ok(threads) = get_thread_ids(pid) else {
                    thread::sleep(Duration::from_millis(1000));
                    continue;
                };
                all_trackers = threads
                    .iter()
                    .copied()
                    .filter_map(|tid| {
                        Some((
                            tid,
                            match all_trackers.entry(tid) {
                                Entry::Occupied(o) => o.remove(),
                                Entry::Vacant(_) => UsageTracker::new(pid, tid).ok()?,
                            },
                        ))
                    })
                    .collect();
                let mut top_threads: Vec<_> = all_trackers
                    .iter()
                    .filter_map(|(tid, tracker)| {
                        Some((*tid, tracker.clone().try_calculate().ok()?))
                    })
                    .collect();
                top_threads
                    .sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap_or(cmp::Ordering::Equal));
                top_threads.truncate(5);
                top_trackers = top_threads
                    .into_iter()
                    .filter_map(|(tid, _)| match top_trackers.entry(tid) {
                        Entry::Occupied(o) => Some((tid, o.remove())),
                        Entry::Vacant(_) => Some((tid, UsageTracker::new(pid, tid).ok()?)),
                    })
                    .collect();
                last_full_update = Instant::now();
            }

            let mut max_usage = 0.0;
            let mut max_tid = None;
            for (tid, tracker) in &mut top_trackers {
                if let Ok(usage) = tracker.try_calculate() {
                    if usage > max_usage {
                        max_usage = usage;
                        max_tid = Some(*tid);
                    }
                }
            }

            if let Some(tid) = max_tid {
                // 发送元组 (tid)
                max_usage_tid.send(tid).unwrap();
            }
        }

        thread::sleep(Duration::from_millis(1000));
    }
}

fn get_thread_ids(pid: pid_t) -> Result<Vec<pid_t>> {
    let proc_path = format!("/proc/{pid}/task");
    Ok(fs::read_dir(proc_path)?
        .filter_map(|entry| {
            entry
                .ok()
                .and_then(|e| e.file_name().to_string_lossy().parse::<pid_t>().ok())
        })
        .collect())
}

// fn get_thread_tids(task_map: &HashMap<pid_t, CompactString>, prefix: &str) -> Vec<pid_t> {
// task_map
// .iter()
// .filter_map(|(&tid, name)| {
// if name.starts_with(prefix) {
// Some(tid)
// } else {
// None
// }
// })
// .collect()
// }

fn get_thread_cpu_time(pid: pid_t, tid: pid_t) -> Result<u64> {
    let stat_path = format!("/proc/{pid}/task/{tid}/stat");
    let stat_content = fs::read_to_string(stat_path)?;
    let parts: Vec<&str> = stat_content.split_whitespace().collect();
    let utime = parts[13].parse::<u64>().unwrap_or(0);
    let stime = parts[14].parse::<u64>().unwrap_or(0);
    Ok(utime + stime)
}
