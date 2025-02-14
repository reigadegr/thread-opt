// From shadow3aaa fas-rs
use super::usage_tracker::UsageTracker;
use hashbrown::HashMap;
use libc::pid_t;

pub fn get_high_usage_tids(target_tids: &[pid_t]) -> (pid_t, pid_t) {
    let all_trackers: HashMap<pid_t, u64> = target_tids
        .iter()
        .copied()
        .map(|tid| (tid, UsageTracker::new(tid).try_calculate()))
        .collect();

    let (tid1, tid2) = get_top_usage_tid(&all_trackers);
    (tid1, tid2)
}

fn get_top_usage_tid(trackers: &HashMap<pid_t, u64>) -> (pid_t, pid_t) {
    let mut tid1 = -1;
    let mut tid2 = -1;
    let mut usage1: u64 = 0;
    let mut usage2: u64 = 0;

    for (tid, &cputime) in trackers {
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
