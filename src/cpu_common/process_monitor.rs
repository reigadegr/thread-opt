//From shadow3aaa fas-rs
use crate::policy::usage::UNNAME_TIDS;
use anyhow::{anyhow, Result};
use flume::{Receiver, Sender};
use hashbrown::{hash_map::Entry, HashMap};
use libc::pid_t;
use likely_stable::likely;

#[cfg(debug_assertions)]
use log::debug;
use std::{
    cmp, fs, thread,
    time::{Duration, Instant},
};

#[derive(Debug, Clone)]
struct UsageTracker {
    tid: pid_t,
}

impl UsageTracker {
    const fn new(tid: pid_t) -> Self {
        Self { tid }
    }

    fn try_calculate(&self) -> u64 {
        get_thread_cpu_time(self.tid)
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

        if let Some(_pid) = current_pid {
            if last_full_update.elapsed() > Duration::from_millis(1600) {
                let Ok(threads) = get_target_tids(rx) else {
                    #[cfg(debug_assertions)]
                    debug!("错误获取tids，休眠后跳过");
                    thread::sleep(Duration::from_millis(400));
                    continue;
                };

                all_trackers = threads
                    .iter()
                    .copied()
                    .map(|tid| {
                        (
                            tid,
                            match all_trackers.entry(tid) {
                                Entry::Occupied(o) => o.remove(),
                                Entry::Vacant(_) => UsageTracker::new(tid),
                            },
                        )
                    })
                    .collect();

                let top_threads = get_top_usage_tid(&mut all_trackers, 5);

                top_trackers = top_threads
                    .into_iter()
                    .map(|(tid, _)| {
                        let tracker = all_trackers
                            .get(&tid)
                            .cloned()
                            .unwrap_or_else(|| UsageTracker::new(tid));
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
) -> Vec<(pid_t, u64)> {
    let mut need_sort: Vec<_> = trackers
        .iter_mut()
        .map(|(tid, tracker)| (*tid, tracker.try_calculate()))
        .collect();
    need_sort.sort_unstable_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap_or(cmp::Ordering::Equal));
    need_sort.truncate(cut_num);
    need_sort
}

fn get_target_tids(rx: &Receiver<Vec<pid_t>>) -> Result<Vec<pid_t>> {
    rx.try_recv().map_or_else(
        |_| {
            #[cfg(debug_assertions)]
            debug!("通道为空，返回一个错误");
            Err(anyhow!("Cannot get tids."))
        },
        // |tids| Ok(tids),
        Ok,
    )
}

fn get_thread_cpu_time(tid: pid_t) -> u64 {
    let stat_path = format!("/proc/{tid}/schedstat");
    let stat_content = fs::read_to_string(stat_path).unwrap_or_else(|_| String::from("0"));
    let parts: Vec<&str> = stat_content.split_whitespace().collect();
    parts[0].parse::<u64>().unwrap_or(0)
}
