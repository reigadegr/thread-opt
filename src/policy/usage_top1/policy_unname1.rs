use super::common::Policy;
use crate::policy::pkg_cfg::StartArgs;
#[cfg(debug_assertions)]
use log::debug;

const TOP: [&str; 1] = ["Thread-"];
const ONLY6: [&str; 2] = ["RHIThread", "RenderThread"];
const ONLY7: [&str; 0] = [];
const MIDDLE: [&str; 0] = [];
const BACKEND: [&str; 0] = [];

pub fn start_task(args: &mut StartArgs) {
    #[cfg(debug_assertions)]
    let start = std::time::Instant::now();

    args.controller.update_max_usage_tid();
    let Some(tid1) = args.controller.first_max_tid() else {
        return;
    };

    Policy::new(&TOP, &ONLY6, &ONLY7, &MIDDLE, &BACKEND).execute_policy(args.task_map, tid1);

    #[cfg(debug_assertions)]
    {
        let end = start.elapsed();

        debug!(
            "单线程:一轮绑定核心完成时间: {:?} 数组长度{}",
            end,
            args.task_map.len()
        );
    }
}
