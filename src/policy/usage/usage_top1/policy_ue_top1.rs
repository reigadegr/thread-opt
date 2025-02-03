use super::common::Policy;
use crate::policy::{
    pkg_cfg::StartArgs,
    usage::{get_thread_tids, UNNAME_TIDS},
};
#[cfg(debug_assertions)]
use log::debug;
use std::time::Duration;

const TOP: [&[u8]; 0] = [];
const ONLY6: [&[u8]; 2] = [b"RHIThread", b"RenderThread"];
const ONLY7: [&[u8]; 0] = [];
const MIDDLE: [&[u8]; 0] = [];
const BACKEND: [&[u8]; 0] = [];

pub fn start_task(args: &mut StartArgs) {
    args.controller.init_game(*args.pid);
    // 获取全局通道的发送端
    let tx = &UNNAME_TIDS.0;

    loop {
        let pid = args.activity_utils.top_app_utils.get_pid();
        if pid != args.pid {
            args.controller.init_default();
            return;
        }

        let task_map = args.activity_utils.tid_utils.get_task_map(*pid);

        let unname_tids = get_thread_tids(task_map, b"Thread-");
        #[cfg(debug_assertions)]
        debug!("发送即将开始");
        tx.send(unname_tids).unwrap();
        #[cfg(debug_assertions)]
        debug!("发送已经完毕");

        args.controller.update_max_usage_tid();
        let Some(tid1) = args.controller.first_max_tid() else {
            std::thread::sleep(Duration::from_millis(100));
            continue;
        };

        Policy::new(&TOP, &ONLY6, &ONLY7, &MIDDLE, &BACKEND).execute_policy(task_map, tid1);

        std::thread::sleep(Duration::from_millis(2000));
    }
}
