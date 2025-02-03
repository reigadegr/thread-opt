use crate::define_policy;
#[cfg(debug_assertions)]
use log::debug;

const TOP: [&str; 0] = [];
const ONLY6: [&str; 1] = ["UnityGfxDeviceW"];
const ONLY7: [&str; 1] = ["UnityMain"];
const MIDDLE: [&str; 2] = ["Thread-", "Job.Worker"];
const BACKEND: [&str; 0] = [];

define_policy!(start_task, &TOP, &ONLY6, &ONLY7, &MIDDLE, &BACKEND);
