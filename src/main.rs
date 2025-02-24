#![no_std]
#![deny(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::non_std_lazy_statics)]

mod activity;
mod cgroup;
mod cpu_common;
mod misc;
mod policy;
mod scheduler;
mod utils;

use misc::init_misc;
use scheduler::Scheduler;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() {
    init_misc();
    Scheduler::new().start_run();
}
