use super::common::Policy;
use crate::policy::{
    pkg_cfg::StartArgs,
    usage::{check_some, get_thread_tids, UNNAME_TIDS},
};
use likely_stable::{likely, unlikely};
#[cfg(debug_assertions)]
use log::debug;
use std::time::Duration;

const TOP: [&[u8]; 0] = [];
const ONLY6: [&[u8]; 0] = [];
const ONLY7: [&[u8]; 1] = [b"UnityMain"];
const MIDDLE: [&[u8]; 0] = [];
const BACKEND: [&[u8]; 0] = [];

pub fn start_task(args: &mut StartArgs) {
    args.controller.init_game(true);
    // 获取全局通道的发送端
    let tx = &UNNAME_TIDS.0;

    let mut finish = false;
    let mut usage_top1 = 0;

    loop {
        let pid = args.activity_utils.top_app_utils.get_pid();
        if unlikely(pid != args.pid) {
            args.controller.init_default();
            return;
        }

        let task_map = args.activity_utils.tid_utils.get_task_map(*pid);
        if likely(finish) {
            Policy::new(&TOP, &ONLY6, &ONLY7, &MIDDLE, &BACKEND)
                .execute_policy(task_map, usage_top1);
            std::thread::sleep(Duration::from_millis(1000));
        } else {
            let unname_tids = get_thread_tids(task_map, b"Thread-");
            #[cfg(debug_assertions)]
            debug!("发送即将开始");
            tx.send(unname_tids).unwrap();
            #[cfg(debug_assertions)]
            debug!("发送已经完毕");
            std::thread::sleep(Duration::from_millis(100));
            args.controller.update_max_usage_tid();
            check_some! {tid1, args.controller.first_max_tid(), "无法获取最大负载tid"};
            usage_top1 = tid1;
            args.controller.init_default();
            finish = true;
            #[cfg(debug_assertions)]
            debug!("计算后最终结果为:{usage_top1}\n");
            continue;
        }

        std::thread::sleep(Duration::from_millis(1000));
    }
}
