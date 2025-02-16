// From shadow3aaa fas-rs
use super::usage_tracker::UsageTracker;
use hashbrown::HashMap;
use libc::pid_t;

pub fn get_top1_tid(target_tids: &[pid_t]) -> pid_t {
    let all_trackers: HashMap<pid_t, u64> = target_tids
        .iter()
        .map(|&tid| (tid, UsageTracker::new(tid).try_calculate()))
        .collect();

    find_top1_tids(&all_trackers)
}

fn find_top1_tids(trackers: &HashMap<pid_t, u64>) -> pid_t {
    let mut tid1 = -1;
    let mut usage1: u64 = 0;

    for (&tid, &cputime) in trackers {
        if cputime > usage1 {
            usage1 = cputime;
            tid1 = tid;
        }
    }
    tid1
}

pub fn get_top2_tids(target_tids: &[pid_t]) -> (pid_t, pid_t) {
    let all_trackers: HashMap<pid_t, u64> = target_tids
        .iter()
        .map(|&tid| (tid, UsageTracker::new(tid).try_calculate()))
        .collect();

    find_top2_tids(&all_trackers)
}

fn find_top2_tids(trackers: &HashMap<pid_t, u64>) -> (pid_t, pid_t) {
    let (mut tid1, mut tid2) = (-1, -1);

    let (mut usage1, mut usage2) = (0u64, 0u64);

    for (&tid, &cputime) in trackers {
        if cputime > usage1 {
            // 避免极端情况下获取到的cputime永远比上一个大导致tid2不被赋值
            usage2 = usage1;
            tid2 = tid1;

            usage1 = cputime;
            tid1 = tid;
            continue;
        }

        if cputime > usage2 {
            usage2 = cputime;
            tid2 = tid;
        }
    }
    (tid1, tid2)
}
