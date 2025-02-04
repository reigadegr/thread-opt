//From shadow3aaa fas-rs
use crate::policy::usage::UNNAME_TIDS;
use anyhow::{anyhow, Result};
use flume::{Receiver, Sender};
use hashbrown::{hash_map::Entry, HashMap};
use libc::{pid_t, sysconf, _SC_CLK_TCK};
use likely_stable::likely;
use once_cell::sync::Lazy;

static TICK_PER_SEC: Lazy<i64> = Lazy::new(|| unsafe { sysconf(_SC_CLK_TCK) });

fn get_tick_per_sec() -> &'static i64 {
    &TICK_PER_SEC
}

#[cfg(debug_assertions)]
use log::debug;
use std::{
    cmp, fs, thread,
    time::{Duration, Instant},
};

#[derive(Debug, Clone)]
struct UsageTracker {
    pid: pid_t,
    tid: pid_t,
    last_cputime: u32,
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

    fn try_calculate(&mut self) -> Result<f32> {
        let tick_per_sec = get_tick_per_sec();
        let new_cputime = get_thread_cpu_time(self.pid, self.tid)?;
        let elapsed_ticks = self.read_timer.elapsed().as_secs_f32() * *tick_per_sec as f32;
        self.read_timer = Instant::now();
        let cputime_slice = new_cputime - self.last_cputime;
        self.last_cputime = new_cputime;
        Ok(cputime_slice as f32 / elapsed_ticks)
    }
}

#[derive(Debug)]
pub struct ProcessMonitor {
    sender: Sender<Option<pid_t>>,
    max_usage_tid: Receiver<(pid_t, pid_t)>,
}

impl ProcessMonitor {
    pub fn new() -> Self {
        let (sender, receiver) = flume::bounded(0);
        let (max_usage_tid_sender, max_usage_tid) = flume::unbounded();

        thread::Builder::new()
            .name("UsageCalculater".to_string())
            .spawn(move || {
                monitor_thread(&receiver, &max_usage_tid_sender);
            })
            .unwrap();

        Self {
            sender,
            max_usage_tid,
        }
    }

    pub fn set_pid(&self, pid: Option<pid_t>) {
        self.sender.send(pid).unwrap();
    }

    pub fn update_max_usage_tid(&self) -> Option<(pid_t, pid_t)> {
        self.max_usage_tid.try_iter().last()
    }
}

fn monitor_thread(receiver: &Receiver<Option<pid_t>>, max_usage_tid: &Sender<(pid_t, pid_t)>) {
    let mut current_pid = None;
    let mut last_full_update = Instant::now();
    let mut all_trackers = HashMap::new();
    let mut top_trackers = HashMap::new();
    let rx = &UNNAME_TIDS.1;

    loop {
        if let Ok(pid) = receiver.try_recv() {
            current_pid = pid;
            all_trackers.clear();
            top_trackers.clear();
        }

        if let Some(pid) = current_pid {
            if last_full_update.elapsed() > Duration::from_millis(1600) {
                #[cfg(debug_assertions)]
                debug!("开始全量更新tid");
                let Ok(threads) = get_target_tids(rx) else {
                    #[cfg(debug_assertions)]
                    debug!("错误获取，休眠后跳过");
                    thread::sleep(Duration::from_millis(400));
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
                let top_threads = get_top_usage_tid(&mut all_trackers, 5);
                top_trackers = top_threads
                    .into_iter()
                    .map(|(tid, _)| {
                        let tracker = all_trackers.get(&tid).cloned().unwrap_or_else(|| {
                            #[cfg(debug_assertions)]
                            debug!("需要重新创建跟踪对象，bug原因未知");
                            UsageTracker::new(pid, tid).expect("Failed to create UsageTracker")
                        });
                        (tid, tracker)
                    })
                    .collect();

                last_full_update = Instant::now();
            }
            let top_two = get_top_usage_tid(&mut top_trackers, 2);
            if likely(top_two.len() > 1) {
                max_usage_tid.send((top_two[0].0, top_two[1].0)).unwrap();
            }
            #[cfg(debug_assertions)]
            debug!("计算完一轮了");
        } else {
            all_trackers.clear();
            top_trackers.clear();
            thread::sleep(Duration::from_millis(1314));
        }
        thread::sleep(Duration::from_millis(521));
    }
}

fn get_top_usage_tid(
    trackers: &mut HashMap<pid_t, UsageTracker>,
    cut_num: usize,
) -> Vec<(pid_t, Option<f32>)> {
    let mut need_sort: Vec<_> = trackers
        .iter_mut()
        .map(|(tid, tracker)| (*tid, tracker.try_calculate().ok()))
        .collect();
    need_sort.sort_unstable_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap_or(cmp::Ordering::Equal));
    need_sort.truncate(cut_num);
    need_sort
}

fn get_target_tids(rx: &Receiver<Vec<pid_t>>) -> Result<Vec<pid_t>> {
    #[cfg(debug_assertions)]
    debug!("开始计算负载喵，开始接收数据");
    rx.try_recv().map_or_else(
        |_| {
            #[cfg(debug_assertions)]
            debug!("通道为空，返回一个错误");
            Err(anyhow!("Cannot get tids."))
        },
        |tids| {
            #[cfg(debug_assertions)]
            debug!("成功获取，这是收到的未命名的tids:{tids:?}");
            Ok(tids)
        },
    )
}

fn get_thread_cpu_time(pid: pid_t, tid: pid_t) -> Result<u32> {
    let stat_path = format!("/proc/{pid}/task/{tid}/stat");
    let stat_content = fs::read_to_string(stat_path)?;
    let parts: Vec<&str> = stat_content.split_whitespace().collect();
    let utime = parts[13].parse::<u32>().unwrap_or(0);
    let stime = parts[14].parse::<u32>().unwrap_or(0);
    Ok(utime + stime)
}
