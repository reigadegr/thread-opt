use crate::policy::name_match::common::Policy;
use crate::policy::pkg_cfg::StartArgs;
use compact_str::CompactString;
use hashbrown::HashMap;
use libc::pid_t;

const TOP: [&str; 0] = [];
const ONLY6: [&str; 2] = ["RHIThread", "RenderThread"];
const ONLY7: [&str; 1] = ["GameThread"];
const MIDDLE: [&str; 0] = [];
const BACKEND: [&str; 0] = [];

pub fn start_task(args: &StartArgs) {
    Policy::new(&TOP, &ONLY6, &ONLY7, &MIDDLE, &BACKEND).execute_policy(args.task_map);
}
