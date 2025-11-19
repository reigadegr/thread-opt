// From shadow3aaa fas-rs
use super::usage_tracker::UsageTracker;
use futures::future::join_all;
use std::collections::HashMap;

pub async fn get_top1_tid(target_tids: &[i32]) -> i32 {
    // 把每个 tid 变成异步任务
    let tasks: Vec<_> = target_tids
        .iter()
        .map(|&tid| async move { (tid, UsageTracker::new(tid).try_calculate().await) })
        .collect();

    // 并发等待全部完成
    let all_trackers: HashMap<i32, u64> = join_all(tasks).await.into_iter().collect();

    find_top1_tids(&all_trackers)
}

fn find_top1_tids(trackers: &HashMap<i32, u64>) -> i32 {
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

pub async fn get_top2_tids(target_tids: &[i32]) -> (i32, i32) {
    let tasks: Vec<_> = target_tids
        .iter()
        .map(|&tid| async move { (tid, UsageTracker::new(tid).try_calculate().await) })
        .collect();
    let all_trackers: HashMap<i32, u64> = join_all(tasks).await.into_iter().collect();
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
