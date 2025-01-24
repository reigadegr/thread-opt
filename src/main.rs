#![deny(clippy::pedantic)]
#![warn(clippy::nursery)]
mod activity;
mod affinity_policy;
mod affinity_utils;
mod cgroup;
mod fs_utils;
mod looper;
mod misc;
use crate::cgroup::{analysis::analysis_cgroup_new, group_info::print_group_core};
use looper::Looper;
use misc::logger::init_misc;

fn main() {
    init_misc();
    let _ = analysis_cgroup_new();
    print_group_core();
    Looper::new().enter_loop();
}
