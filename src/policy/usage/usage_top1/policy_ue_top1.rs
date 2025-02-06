use super::common::Policy;
use crate::policy::{
    pkg_cfg::StartArgs,
    usage::{get_thread_tids, UNNAME_TIDS},
};
use hashbrown::HashSet; // 修改为 HashSet
use libc::pid_t;
use likely_stable::{likely, unlikely};
#[cfg(debug_assertions)]
use log::debug;
use std::time::Duration;

const TOP: [&[u8]; 0] = [];
const ONLY6: [&[u8]; 2] = [b"RHIThread", b"RenderThread"];
const ONLY7: [&[u8]; 0] = [];
const MIDDLE: [&[u8]; 0] = [];
const BACKEND: [&[u8]; 0] = [];

pub fn start_task(args: &mut StartArgs) {
    args.controller.init_game(true);
    // 获取全局通道的发送端
    let tx = &UNNAME_TIDS.0;

    // 创建一个HashSet<pid_t>
    let mut high_usage_tids: Option<HashSet<pid_t>> = Some(HashSet::new());

    let mut finish = false;
    let mut usage_top1 = 0;
    let mut insert_count: u8 = 0;

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

            if likely(insert_count < 25) {
                if let Some(set) = high_usage_tids.as_mut() {
                    set.insert(tid1); // 插入 tid1
                    #[cfg(debug_assertions)]
                    debug!("负载第一高:{tid1}");

                    if unlikely(set.len() > 1) {
                        #[cfg(debug_assertions)]
                        debug!("检测到集合长度大于1，重新vote");
                        set.clear();
                        insert_count = 10;
                        continue;
                    }

                    insert_count += 1;
                }

                Policy::new(&TOP, &ONLY6, &ONLY7, &MIDDLE, &BACKEND).execute_policy(task_map, tid1);
            } else {
                usage_top1 = tid1;

                args.controller.init_default();
                finish = true;
                high_usage_tids = None;
                #[cfg(debug_assertions)]
                debug!("计算后最终结果为:{usage_top1}\n");
                continue;
            }
        }

        std::thread::sleep(Duration::from_millis(1900));
    }
}
