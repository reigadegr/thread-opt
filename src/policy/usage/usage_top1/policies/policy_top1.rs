use crate::policy::{
    pkg_cfg::StartArgs,
    usage::usage_top1::{macro_common::Policy, top1_macro_init},
};

const TOP: [&[u8]; 0] = [];
const ONLY6: [&[u8]; 1] = [b"RHIThread"];
const ONLY7: [&[u8]; 0] = [];
const MIDDLE: [&[u8]; 1] = [b"RenderThread"];
const BACKEND: [&[u8]; 0] = [];

top1_macro_init!(b"Thread-", Only7);
