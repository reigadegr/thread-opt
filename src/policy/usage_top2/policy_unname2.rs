use crate::policy::{pkg_cfg::StartArgs, usage_top2::common::execute_policy};
#[cfg(debug_assertions)]
use log::debug;

pub fn start_task(args: &mut StartArgs) {
    #[cfg(debug_assertions)]
    let start = std::time::Instant::now();

    args.controller.update_max_usage_tid();
    let Some(tid1) = args.controller.first_max_tid() else {
        return;
    };

    let Some(tid2) = args.controller.second_max_tid() else {
        return;
    };
    execute_policy(args.task_map, tid1, tid2);

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
