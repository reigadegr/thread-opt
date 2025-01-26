#![deny(clippy::pedantic)]
#![warn(clippy::nursery)]
mod activity;
mod cgroup;
mod looper;
mod misc;
mod policy;
mod utils;
use cgroup::{analysis::analysis_cgroup_new, group_info::print_group_core};
use looper::Looper;
use misc::{init_misc, working_in_background};

fn main() {
    init_misc();
    let _ = analysis_cgroup_new();
    working_in_background();
    print_group_core();
    Looper::new().enter_loop();
}
