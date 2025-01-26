use crate::policy::name_match::common::Policy;
use compact_str::CompactString;
use hashbrown::HashMap;
use libc::pid_t;

const TOP: [&str; 0] = [];
const ONLY6: [&str; 1] = ["UnityGfxDeviceW"];
const ONLY7: [&str; 1] = ["UnityMain"];
const MIDDLE: [&str; 2] = ["Thread-", "Job.Worker"];
const BACKEND: [&str; 0] = [];

// pub fn start_task(task_map: &HashMap<pid_t, CompactString>) {
// Policy::new(&TOP, &ONLY6, &ONLY7, &MIDDLE, &BACKEND).execute_policy(&task_map);
// }

pub fn start_task(task_map: &HashMap<pid_t, CompactString>) {
    // let task_map = Arc::new(task_map);
    Policy::new(&TOP, &ONLY6, &ONLY7, &MIDDLE, &BACKEND).execute_policy(task_map);
}
