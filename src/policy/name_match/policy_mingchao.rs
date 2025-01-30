use crate::policy::name_match::common::Policy;
use crate::policy::pkg_cfg::StartArgs;
use log::debug;

const TOP: [&str; 1] = ["Pool"];
const ONLY6: [&str; 2] = ["RHIThread", "RenderThread"];
const ONLY7: [&str; 1] = ["GameThread"];
const MIDDLE: [&str; 0] = [];
const BACKEND: [&str; 0] = [];

pub fn start_task(args: &mut StartArgs) {
    #[cfg(debug_assertions)]
    let start = std::time::Instant::now();
    args.controller.init_default();
    Policy::new(&TOP, &ONLY6, &ONLY7, &MIDDLE, &BACKEND).execute_policy(args.task_map);
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
