use super::super::name_match_init;

const TOP: [&[u8]; 1] = [b"Pool"];
const ONLY6: [&[u8]; 1] = [b"RHIThread"];
const ONLY7: [&[u8]; 1] = [b"GameThread"];
const MIDDLE: [&[u8]; 1] = [b"RenderThread"];
const BACKEND: [&[u8]; 0] = [];

name_match_init!();
