use super::name_match_policy;

const TOP: [&str; 1] = ["Pool"];
const ONLY6: [&str; 2] = ["RHIThread", "RenderThread"];
const ONLY7: [&str; 1] = ["GameThread"];
const MIDDLE: [&str; 0] = [];
const BACKEND: [&str; 0] = [];

name_match_policy!(start_task, &TOP, &ONLY6, &ONLY7, &MIDDLE, &BACKEND);
