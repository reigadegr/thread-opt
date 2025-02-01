#![deny(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(
    clippy::module_name_repetitions,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::cast_possible_wrap
)]

mod activity;
mod cgroup;
mod cpu_common;
mod misc;
mod policy;
mod scheduler;
mod utils;
use crate::activity::get_top_tid::TopAppUtils;
use cgroup::{analysis::analysis_cgroup_new, group_info::print_group_core};

use misc::init_misc;
use scheduler::Scheduler;

fn main() {
    init_misc();
    let _ = analysis_cgroup_new();
    print_group_core();
    // Scheduler::new().start_run();
    let mut top_app_utils = TopAppUtils::new();
    let mut scheduler = Scheduler::new(&mut top_app_utils);
    scheduler.start_run();
}
