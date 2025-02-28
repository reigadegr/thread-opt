use super::super::name_match_init;

const TOP: &[&[u8]] = &[b"Pool"];
const ONLY6: &[&[u8]] = &[b"RHIThread"];
const ONLY7: &[&[u8]] = &[b"GameThread"];
const MIDDLE: &[&[u8]] = &[b"RenderThread"];
const BACKEND: &[&[u8]] = &[];

name_match_init!();
