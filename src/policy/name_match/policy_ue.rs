use super::name_match_policy;

const TOP: [&[u8]; 1] = [b"Pool"];
const ONLY6: [&[u8]; 1] = [b"RHIThread"];
const ONLY7: [&[u8]; 1] = [b"GameThread"];
const MIDDLE: [&[u8]; 1] = [b"RenderThread"];
const BACKEND: [&[u8]; 0] = [];

name_match_policy!(start_task, &TOP, &ONLY6, &ONLY7, &MIDDLE, &BACKEND);
