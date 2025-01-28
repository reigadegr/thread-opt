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
mod looper;
mod misc;
mod policy;
mod utils;
use cgroup::{analysis::analysis_cgroup_new, group_info::print_group_core};
use looper::Looper;
use misc::init_misc;

fn main() {
    init_misc();
    let _ = analysis_cgroup_new();
    print_group_core();
    Looper::new().enter_loop();
}
