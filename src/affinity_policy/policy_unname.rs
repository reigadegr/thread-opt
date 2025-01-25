use super::policy_common::{execute_task, get_cmd_type, CmdType};
use crate::affinity_utils::bind_thread_to_cpu;
use libc::pid_t;

const TOP: [&str; 1] = ["Thread-"];
const ONLY6: [&str; 0] = [];
const ONLY7: [&str; 1] = [" "];
const MIDDLE: [&str; 2] = ["RHIThread", "RenderThread"];
const BACKEND: [&str; 0] = [];

pub fn start_task(tid: pid_t, thread: &str) {
    let thread_type = get_cmd_type(thread, &TOP, &ONLY6, &ONLY7, &MIDDLE, &BACKEND);
    execute_task(&thread_type, tid);
}
