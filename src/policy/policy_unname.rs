use crate::policy::common::Policy;
use crate::policy::pkg_cfg::StartArgs;
use compact_str::CompactString;
use hashbrown::HashMap;
use libc::pid_t;
// #[cfg(debug_assertions)]
// use log::debug;

const TOP: [&str; 1] = ["Thread-"];
const ONLY6: [&str; 0] = [];
const ONLY7: [&str; 0] = [];
const MIDDLE: [&str; 2] = ["RHIThread", "RenderThread"];
const BACKEND: [&str; 0] = [];

pub fn start_task(args: &StartArgs) {
    // #[cfg(debug_assertions)]
    // {
    // let thread_tids = get_thread_tids(task_map, "Thread-");
    // debug!("Thread- TIDs: {thread_tids:?}");
    // }

    Policy::new(&TOP, &ONLY6, &ONLY7, &MIDDLE, &BACKEND).execute_policy(args.task_map);
}

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
