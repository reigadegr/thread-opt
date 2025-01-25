use crate::policy::name_match::policy_common::Policy;
use hashbrown::HashMap;
use libc::pid_t;

const TOP: [&str; 0] = [];
const ONLY6: [&str; 2] = ["RHIThread", "RenderThread"];
const ONLY7: [&str; 1] = ["GameThread"];
const MIDDLE: [&str; 0] = [];
const BACKEND: [&str; 0] = [];

pub fn start_task(task_map: &HashMap<pid_t, String>) {
    let policy = Policy::new(&TOP, &ONLY6, &ONLY7, &MIDDLE, &BACKEND);
    policy.execute_policy(task_map);
}
