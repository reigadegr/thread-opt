use super::common::Policy;
use crate::policy::pkg_cfg::StartArgs;
#[cfg(debug_assertions)]
use log::debug;
use std::time::Duration;

const TOP: [&str; 1] = ["Thread-"];
const ONLY6: [&str; 2] = ["RHIThread", "RenderThread"];
const ONLY7: [&str; 0] = [];
const MIDDLE: [&str; 0] = [];
const BACKEND: [&str; 0] = [];

pub fn start_task(args: &mut StartArgs) {
    args.controller.init_game(*args.pid);
    loop {
        let pid = args.top_app_utils.get_pid();
        if pid != args.pid {
            args.controller.init_default();
            return;
        }
        #[cfg(debug_assertions)]
        let start = std::time::Instant::now();

        args.controller.update_max_usage_tid();
        let Some(tid1) = args.controller.first_max_tid() else {
            continue;
        };

        let task_map = args.tid_utils.get_task_map(*pid);
        Policy::new(&TOP, &ONLY6, &ONLY7, &MIDDLE, &BACKEND).execute_policy(task_map, tid1);

        #[cfg(debug_assertions)]
        {
            let end = start.elapsed();

            debug!(
                "单线程:一轮绑定核心完成时间: {:?} 数组长度{}",
                end,
                task_map.len()
            );
        }
        std::thread::sleep(Duration::from_millis(2000));
    }
}
