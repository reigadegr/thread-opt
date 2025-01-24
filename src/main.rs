mod activity;
mod affinity_policy;
mod affinity_utils;
mod fs_utils;
mod looper;
mod misc;

use crate::affinity_utils::{analysis::analysis_cgroup_new, group_info::print_group_core};
use looper::Looper;
use misc::logger::init_misc;

fn main() -> anyhow::Result<()> {
    init_misc();
    let _ = analysis_cgroup_new();
    print_group_core();
    Looper::new().enter_loop();
    Ok(())
}
