mod activity;
mod affinity_policy;
mod affinity_utils;
mod fs_utils;
mod looper;
mod misc;
use crate::affinity_utils::analysis::analysis_cgroup_new;
use fs_utils::init_working_directory;
use looper::Looper;
use misc::logger::init_misc;

fn main() -> anyhow::Result<()> {
    init_misc();
    let _ = init_working_directory();
    let _ = analysis_cgroup_new();
    Looper::new().enter_loop();
    Ok(())
}
