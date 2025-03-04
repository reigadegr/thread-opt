#![no_std]
#![no_main]
#![warn(clippy::nursery, clippy::pedantic)]
#![allow(
    clippy::non_std_lazy_statics,
    clippy::similar_names,
    clippy::missing_safety_doc,
    clippy::missing_panics_doc
)]

mod activity;
mod cgroup;
mod config;
mod cpu_common;
mod misc;
mod policy;
mod scheduler;
mod utils;

use core::{ffi::CStr, slice};
use libc::{c_char, c_int};
use misc::init_misc;
use scheduler::Scheduler;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[unsafe(no_mangle)]
unsafe extern "C" fn main(argc: c_int, argv: *const *const c_char) {
    init_misc();
    let args = unsafe { slice::from_raw_parts(argv, argc.try_into().unwrap_or(0)) };

    for arg in args {
        if let Ok(s) = unsafe { CStr::from_ptr(*arg) }.to_str() {
            log::info!("命令行参数{s}");
        }
    }
    Scheduler::new().start_run();
}
