use super::common::execute_policy;
use crate::policy::pkg_cfg::StartArgs;
#[cfg(debug_assertions)]
use log::debug;
use std::time::Duration;

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

        let Some(tid2) = args.controller.second_max_tid() else {
            continue;
        };
        let task_map = args.tid_utils.get_task_map(*pid);
        execute_policy(task_map, tid1, tid2);

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
