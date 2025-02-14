// From shadow3aaa fas-rs
use super::usage_tracker::UsageTracker;
use crate::policy::usage::UNNAME_TIDS;
use anyhow::{anyhow, Result};
use flume::{Receiver, Sender};
use hashbrown::{hash_map::Entry, HashMap};
use libc::pid_t;

#[cfg(debug_assertions)]
use log::debug;
use std::time::Duration;

#[derive(Debug)]
pub struct ProcessMonitor {
    sender: Sender<Option<bool>>,
    max_usage_tid: Receiver<(pid_t, pid_t)>,
}

impl ProcessMonitor {
    pub fn new() -> Self {
        let (sender, receiver) = flume::bounded(0);
        let (max_usage_tid_sender, max_usage_tid) = flume::bounded(0);

        std::thread::Builder::new()
            .name("UsageSampler".to_string())
            .spawn(move || {
                monitor_thread(&receiver, &max_usage_tid_sender);
            })
            .unwrap();

        Self {
            sender,
            max_usage_tid,
        }
    }

    pub fn set_work_state(&self, work_state: Option<bool>) {
        self.sender.send(work_state).unwrap();
    }

    pub fn update_max_usage_tid(&self) -> (pid_t, pid_t) {
        #[cfg(debug_assertions)]
        debug!("开始获取最大tid");
        self.max_usage_tid.recv().unwrap()
    }
}

fn monitor_thread(receiver: &Receiver<Option<bool>>, max_usage_tid: &Sender<(pid_t, pid_t)>) {
    let mut work_state = None;
    let mut all_trackers = HashMap::new();
    let rx = &UNNAME_TIDS.1;

    loop {
        std::thread::sleep(Duration::from_millis(1000));
        if let Ok(state) = receiver.try_recv() {
            work_state = state;
            all_trackers.clear();
        }

        if work_state.is_none() {
            continue;
        }

        let Ok(threads) = get_target_tids(rx) else {
            #[cfg(debug_assertions)]
            debug!("错误获取tids，休眠后跳过");
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

        let (tid1, tid2) = get_top_usage_tid(&mut all_trackers);
        max_usage_tid.send((tid1, tid2)).unwrap();
        #[cfg(debug_assertions)]
        debug!("计算完一轮了");
    }
}

pub fn get_high_usage_tids(target_tids: &[pid_t]) -> (pid_t, pid_t) {
    let mut all_trackers = HashMap::new();
    all_trackers = target_tids
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

    let (tid1, tid2) = get_top_usage_tid(&mut all_trackers);
    (tid1, tid2)
}

fn get_top_usage_tid(trackers: &mut HashMap<pid_t, UsageTracker>) -> (pid_t, pid_t) {
    let mut tid1 = -1;
    let mut tid2 = -1;
    let mut usage1: u64 = 0;
    let mut usage2: u64 = 0;

    for (tid, tracker) in trackers {
        let cputime = tracker.try_calculate();
        if cputime > usage1 {
            // 避免极端情况下获取到的cputime永远比上一个大导致tid2不被赋值
            usage2 = usage1;
            tid2 = tid1;

            usage1 = cputime;
            tid1 = *tid;
            continue;
        }

        if cputime > usage2 {
            usage2 = cputime;
            tid2 = *tid;
        }
    }
    (tid1, tid2)
}

fn get_target_tids(rx: &Receiver<Vec<pid_t>>) -> Result<Vec<pid_t>> {
    rx.try_recv().map_or_else(
        |_| Err(anyhow!("Cannot get tids.")),
        // |tids| Ok(tids),
        Ok,
    )
}
