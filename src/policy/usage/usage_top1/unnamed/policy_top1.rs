use crate::policy::usage::usage_top1::macro_common::Policy;
use crate::policy::{
    pkg_cfg::StartArgs,
    usage::{check_some, get_thread_tids, usage_top1::top1_macro_init, UNNAME_TIDS},
};
use likely_stable::{likely, unlikely};
#[cfg(debug_assertions)]
use log::debug;
use std::time::Duration;

const TOP: [&[u8]; 0] = [];
const ONLY6: [&[u8]; 1] = [b"RHIThread"];
const ONLY7: [&[u8]; 0] = [];
const MIDDLE: [&[u8]; 1] = [b"RenderThread"];
const BACKEND: [&[u8]; 0] = [];

top1_macro_init!(b"Thread-");
