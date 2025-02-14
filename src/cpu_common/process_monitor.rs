// From shadow3aaa fas-rs
use super::usage_tracker::UsageTracker;
use hashbrown::{hash_map::Entry, HashMap};
use libc::pid_t;

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
