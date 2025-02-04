#![deny(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::cast_precision_loss, clippy::non_std_lazy_statics)]

mod activity;
mod cgroup;
mod cpu_common;
mod misc;
mod policy;
mod scheduler;
mod utils;
use cgroup::group_info::print_group_core;

use misc::init_misc;
use scheduler::Scheduler;

fn main() {
    init_misc();
    print_group_core();
    Scheduler::new().start_run();
}
