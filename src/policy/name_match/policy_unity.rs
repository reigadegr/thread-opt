use super::name_match_policy;

const TOP: [&[u8]; 0] = [];
const ONLY6: [&[u8]; 1] = [b"UnityGfxDeviceW"];
const ONLY7: [&[u8]; 1] = [b"UnityMain"];
const MIDDLE: [&[u8]; 2] = [b"Thread-", b"Job.Worker"];
const BACKEND: [&[u8]; 0] = [];

name_match_policy!(start_task, &TOP, &ONLY6, &ONLY7, &MIDDLE, &BACKEND);
