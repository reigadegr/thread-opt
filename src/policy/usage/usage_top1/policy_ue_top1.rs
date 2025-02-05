use super::common::Policy;
use crate::policy::{
    pkg_cfg::StartArgs,
    usage::{get_thread_tids, UNNAME_TIDS},
};
use hashbrown::HashMap;
#[cfg(debug_assertions)]
use log::debug;
use std::time::Duration;

const TOP: [&[u8]; 0] = [];
const ONLY6: [&[u8]; 2] = [b"RHIThread", b"RenderThread"];
const ONLY7: [&[u8]; 0] = [];
const MIDDLE: [&[u8]; 0] = [];
const BACKEND: [&[u8]; 0] = [];

pub fn start_task(args: &mut StartArgs) {
    // 获取全局通道的发送端
    let tx = &UNNAME_TIDS.0;
    args.controller.init_game(*args.pid);
    // 创建一个容量为20的Vec<pid_t>
    let mut high_usage_tids = Vec::with_capacity(10);

    let mut finish = false;

    let mut usage_top1 = 0;

    loop {
        let pid = args.activity_utils.top_app_utils.get_pid();
        if pid != args.pid {
            args.controller.init_default();
            return;
        }

        let task_map = args.activity_utils.tid_utils.get_task_map(*pid);
        if finish {
            Policy::new(&TOP, &ONLY6, &ONLY7, &MIDDLE, &BACKEND)
                .execute_policy(task_map, usage_top1);
            std::thread::sleep(Duration::from_millis(100));
        } else {
            let unname_tids = get_thread_tids(task_map, b"Thread-");
            #[cfg(debug_assertions)]
            debug!("发送即将开始");
            tx.send(unname_tids).unwrap();
            #[cfg(debug_assertions)]
            debug!("发送已经完毕");
            std::thread::sleep(Duration::from_millis(100));
            args.controller.update_max_usage_tid();
            let Some(tid1) = args.controller.first_max_tid() else {
                std::thread::sleep(Duration::from_millis(100));
                continue;
            };

            if high_usage_tids.len() < 10 {
                high_usage_tids.push(tid1);
                #[cfg(debug_assertions)]
                debug!("负载第一高:{tid1}\n");
                Policy::new(&TOP, &ONLY6, &ONLY7, &MIDDLE, &BACKEND).execute_policy(task_map, tid1);
            } else {
                args.controller.init_default();
                let mut tid_counts = HashMap::new();
                for &tid in &high_usage_tids {
                    *tid_counts.entry(tid).or_insert(0) += 1;
                }

                // 按频次排序，取出频次最高的一个tid
                let mut sorted_tids: Vec<_> = tid_counts.into_iter().collect();

                sorted_tids.sort_unstable_by(|a, b| b.1.cmp(&a.1));
                if let Some((sort1, _)) = sorted_tids.first() {
                    usage_top1 = *sort1;
                }

                finish = true;

                high_usage_tids.clear();
                #[cfg(debug_assertions)]
                debug!("计算后最终结果为:{usage_top1}\n");
                continue;
            }
        }

        std::thread::sleep(Duration::from_millis(1900));
    }
}
