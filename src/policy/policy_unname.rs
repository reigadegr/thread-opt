use crate::policy::common::Policy;
use crate::policy::pkg_cfg::StartArgs;
#[cfg(debug_assertions)]
use log::debug;

const TOP: [&str; 1] = ["Thread-"];
const ONLY6: [&str; 0] = [];
const ONLY7: [&str; 0] = [];
const MIDDLE: [&str; 2] = ["RHIThread", "RenderThread"];
const BACKEND: [&str; 0] = [];

pub fn start_task(args: &mut StartArgs) {
    #[cfg(debug_assertions)]
    let start = std::time::Instant::now();
    #[cfg(debug_assertions)]
    {
        args.controller.update_max_usage_tid();
        if let Some(tid) = args.controller.first_max_tid() {
            debug!("Max load thread: {tid}");
        }
        
        if let Some(tid) = args.controller.second_max_tid() {
            debug!("Second load thread: {tid}");
        }
    }

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

// #[cfg(debug_assertions)]
// {
// let thread_tids = get_thread_tids(task_map, "Thread-");
// debug!("Thread- TIDs: {thread_tids:?}");
// }

// #[cfg(debug_assertions)]
// fn get_thread_tids(task_map: &HashMap<pid_t, CompactString>, prefix: &str) -> Vec<pid_t> {
// task_map
// .iter()
// .filter_map(|(&tid, name)| {
// if name.starts_with(prefix) {
// Some(tid)
// } else {
// None
// }
// })
// .collect()
// }
