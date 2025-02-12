use super::name_match_init;

const TOP: [&[u8]; 0] = [];
const ONLY6: [&[u8]; 0] = [];
const ONLY7: [&[u8]; 1] = [b"GLThread"];
const MIDDLE: [&[u8]; 0] = [];
const BACKEND: [&[u8]; 0] = [];

name_match_init!();
