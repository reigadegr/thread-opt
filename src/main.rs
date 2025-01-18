mod activity;
mod fs_utils;
mod looper;
use crate::fs_utils::analysis_cgroup;
mod misc;
use log::info;
use looper::Looper;
use misc::logger::init_misc;

fn main() -> anyhow::Result<()> {
    init_misc();
    analysis_cgroup();
    info!("Hello, world!");
    // Looper::new().enter_loop();
    Ok(())
}
