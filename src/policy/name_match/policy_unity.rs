use super::name_match_policy;

const TOP: [&str; 0] = [];
const ONLY6: [&str; 1] = ["UnityGfxDeviceW"];
const ONLY7: [&str; 1] = ["UnityMain"];
const MIDDLE: [&str; 2] = ["Thread-", "Job.Worker"];
const BACKEND: [&str; 0] = [];

name_match_policy!(start_task, &TOP, &ONLY6, &ONLY7, &MIDDLE, &BACKEND);
