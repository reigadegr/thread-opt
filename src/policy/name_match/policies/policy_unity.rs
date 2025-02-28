use super::super::name_match_init;

const TOP: &[&[u8]] = &[];
const ONLY6: &[&[u8]] = &[b"UnityGfx"];
const ONLY7: &[&[u8]] = &[b"UnityMain"];
const MIDDLE: &[&[u8]] = &[b"Thread-", b"Job.Worker"];
const BACKEND: &[&[u8]] = &[];

name_match_init!();
