// From shadow3aaa fas-rs
use super::usage_tracker::UsageTracker;
use rayon::prelude::*;
use std::collections::HashMap;

pub fn get_top1_tid(target_tids: &[i32]) -> i32 {
    target_tids
        .par_iter() // 并行采样
        .map(|&tid| (tid, UsageTracker::new(tid).try_calculate()))
        .reduce_with(|(t1, u1), (t2, u2)| if u1 > u2 { (t1, u1) } else { (t2, u2) })
        .map_or(-1, |(tid, _)| tid)
}

pub fn get_top2_tids(target_tids: &[i32]) -> (i32, i32) {
    let all_trackers: HashMap<i32, u64> = target_tids
        .iter()
        .map(|&tid| (tid, UsageTracker::new(tid).try_calculate()))
        .collect();

    find_top2_tids(&all_trackers)
}

fn find_top2_tids(trackers: &HashMap<i32, u64>) -> (i32, i32) {
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
