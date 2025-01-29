use crate::policy::name_match::common::Policy;
use crate::policy::pkg_cfg::StartArgs;

const TOP: [&str; 0] = [];
const ONLY6: [&str; 1] = ["UnityGfxDeviceW"];
const ONLY7: [&str; 1] = ["UnityMain"];
const MIDDLE: [&str; 2] = ["Thread-", "Job.Worker"];
const BACKEND: [&str; 0] = [];

pub fn start_task(args: &StartArgs) {
    Policy::new(&TOP, &ONLY6, &ONLY7, &MIDDLE, &BACKEND).execute_policy(args.task_map);
}
