use crate::policy::name_match::policy_common::{execute_task, get_cmd_type};
use ahash::AHashMap;
use libc::pid_t;

const TOP: [&str; 1] = [" "];
const ONLY6: [&str; 1] = ["UnityGfxDeviceW"];
const ONLY7: [&str; 1] = ["UnityMain"];
const MIDDLE: [&str; 2] = ["Thread-", "Job.Worker"];
const BACKEND: [&str; 0] = [];

pub fn start_task(task_map: &AHashMap<pid_t, String>) {
    for (tid, comm) in task_map {
        let thread_type = get_cmd_type(comm, &TOP, &ONLY6, &ONLY7, &MIDDLE, &BACKEND);
        execute_task(&thread_type, *tid);
    }
}
