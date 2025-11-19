// From shadow3aaa fas-rs
use super::usage_tracker::UsageTracker;
use rayon::prelude::*;
// std::collections::HashMap 不再需要

pub fn get_top1_tid(target_tids: &[i32]) -> i32 {
    target_tids
        .par_iter()
        .map(|&tid| (tid, UsageTracker::new(tid).try_calculate()))
        .reduce_with(|(t1, u1), (t2, u2)| if u1 > u2 { (t1, u1) } else { (t2, u2) })
        .map_or(-1, |(tid, _)| tid)
}

/// 优化后的 `get_top2_tids，使用并行` fold-reduce 模式
pub fn get_top2_tids(target_tids: &[i32]) -> (i32, i32) {
    target_tids
        .par_iter()
        // 1. 并行地创建 (tid, usage) 元组流
        .map(|&tid| (tid, UsageTracker::new(tid).try_calculate()))
        // 2. 在每个线程上折叠出局部的前两名
        //    每个线程的累加器初始值为 ((-1, 0), (-1, 0))，代表 ((tid1, usage1), (tid2, usage2))
        .fold(
            || ((-1, 0u64), (-1, 0u64)),
            |mut acc, (tid, usage)| {
                // acc 是 ((tid1, usage1), (tid2, usage2))
                if usage > acc.0.1 {
                    // 新的 usage 大于当前最大的 usage
                    acc.1 = acc.0; // 原来的第一名变为第二名
                    acc.0 = (tid, usage); // 新项成为第一名
                } else if usage > acc.1.1 {
                    // 新的 usage 介于第一名和第二名之间
                    acc.1 = (tid, usage); // 新项成为第二名
                }
                acc
            },
        )
        // 3. 将所有线程的局部结果归约为全局最终结果
        //    每个 a 和 b 都是 ((tid1, usage1), (tid2, usage2)) 形式的元组
        .reduce_with(|a, b| {
            // 将两个局部结果（共4个候选者）合并，找出真正的全局前两名
            let mut all = [a.0, a.1, b.0, b.1];
            // 按 usage 降序排序。对于4个元素的数组，这个操作非常快。
            all.sort_by_key(|&(_, usage)| std::cmp::Reverse(usage));
            // 返回排序后的前两个
            (all[0], all[1])
        })
        // 4. 处理输入为空的情况，并从最终结果中提取 tid
        .map_or((-1, -1), |((tid1, _), (tid2, _))| (tid1, tid2))
}
