use super::super::name_match_init;

const TOP: [&[u8]; 0] = [];
const ONLY6: [&[u8]; 0] = [];
const ONLY7: [&[u8]; 1] = [b"MainThread"];
const MIDDLE: [&[u8]; 1] = [b"JobThread"];
const BACKEND: [&[u8]; 0] = [];

name_match_init!();
