use crate::policy::name_match::common::Policy;
use compact_str::CompactString;
use hashbrown::HashMap;
use libc::pid_t;

const TOP: [&str; 1] = ["Thread-"];
const ONLY6: [&str; 0] = [];
const ONLY7: [&str; 0] = [];
const MIDDLE: [&str; 2] = ["RHIThread", "RenderThread"];
const BACKEND: [&str; 0] = [];

pub fn start_task(task_map: &HashMap<pid_t, CompactString>) {
    Policy::new(&TOP, &ONLY6, &ONLY7, &MIDDLE, &BACKEND).execute_policy(task_map);
}
