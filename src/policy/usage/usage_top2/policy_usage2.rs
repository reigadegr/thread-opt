use super::common::execute_policy;
use crate::policy::{
    pkg_cfg::StartArgs,
    usage::{get_thread_tids, UNNAME_TIDS},
};
use hashbrown::HashMap;
#[cfg(debug_assertions)]
use log::debug;
use log::info;
use std::time::Duration;

pub fn start_task(args: &mut StartArgs) {
    // 获取全局通道的发送端
    let tx = &UNNAME_TIDS.0;
    args.controller.init_game(*args.pid);
    // 创建一个容量为40的Vec<pid_t>
    let mut high_usage_tids = Vec::with_capacity(40);

    let mut finish = false;

    let mut usage_top1 = 0;
    let mut usage_top2 = 0;

    loop {
        let pid = args.activity_utils.top_app_utils.get_pid();
        if pid != args.pid {
            args.controller.init_default();
            return;
        }

        let task_map = args.activity_utils.tid_utils.get_task_map(*pid);

        if finish {
            execute_policy(task_map, usage_top1, usage_top2, finish);
            std::thread::sleep(Duration::from_millis(100));
        } else {
            let unname_tids = get_thread_tids(task_map, b"Thread-");
            #[cfg(debug_assertions)]
            debug!("发送即将开始");
            tx.send(unname_tids).unwrap();
            #[cfg(debug_assertions)]
            debug!("发送已经完毕，喵等待一段时间计算");
            std::thread::sleep(Duration::from_millis(100));
            args.controller.update_max_usage_tid();
            let Some(tid1) = args.controller.first_max_tid() else {
                #[cfg(debug_assertions)]
                debug!("获取不到first max tid，直接循环");
                std::thread::sleep(Duration::from_millis(100));
                continue;
            };

            let Some(tid2) = args.controller.second_max_tid() else {
                #[cfg(debug_assertions)]
                debug!("获取不到second max tid，直接循环");
                std::thread::sleep(Duration::from_millis(100));
                continue;
            };

            if high_usage_tids.len() < 40 {
                high_usage_tids.push(tid1);
                high_usage_tids.push(tid2);
                #[cfg(debug_assertions)]
                debug!("负载第一高:{tid1}\n第二高:{tid2}");
                execute_policy(task_map, tid1, tid2, finish);
            } else {
                args.controller.init_default();

                let mut tid_counts = HashMap::new();
                for &tid in &high_usage_tids {
                    *tid_counts.entry(tid).or_insert(0) += 1;
                }

                // 按频次排序，取出频次最高的两个tid
                let mut sorted_tids: Vec<_> = tid_counts.into_iter().collect();
                sorted_tids.sort_unstable_by(|a, b| b.1.cmp(&a.1));

                if let Some((sort1, _)) = sorted_tids.first() {
                    usage_top1 = *sort1;
                }

                if let Some((sort2, _)) = sorted_tids.get(1) {
                    usage_top2 = *sort2;
                }
                finish = true;
                // drop(high_usage_tids);
                // #[cfg(debug_assertions)]
                info!("计算后最终结果为:{usage_top1}\n第二高:{usage_top2}");
                continue;
            }
        }

        std::thread::sleep(Duration::from_millis(1900));
    }
}
