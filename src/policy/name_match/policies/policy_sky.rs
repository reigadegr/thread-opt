use super::super::name_match_init;

const TOP: &[&[u8]] = &[];
const ONLY6: &[&[u8]] = &[];
const ONLY7: &[&[u8]] = &[b"MainThread"];
const MIDDLE: &[&[u8]] = &[b"JobThread"];
const BACKEND: &[&[u8]] = &[];

name_match_init!();
