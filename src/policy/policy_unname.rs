use crate::policy::name_match::policy_common::{execute_task, get_cmd_type};
use libc::pid_t;
use std::collections::HashMap;

const TOP: [&str; 1] = ["Thread-"];
const ONLY6: [&str; 0] = [];
const ONLY7: [&str; 1] = [" "];
const MIDDLE: [&str; 2] = ["RHIThread", "RenderThread"];
const BACKEND: [&str; 0] = [];

pub fn start_task(task_map: &HashMap<pid_t, String>) {
    for (tid, comm) in task_map {
        let thread_type = get_cmd_type(comm, &TOP, &ONLY6, &ONLY7, &MIDDLE, &BACKEND);
        execute_task(&thread_type, *tid);
    }
}
