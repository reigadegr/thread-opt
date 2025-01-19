mod activity;
mod fs_utils;
mod looper;
use crate::fs_utils::analysis_cgroup;
use crate::fs_utils::dir_ctrl::get_backend_dir;
use crate::fs_utils::dir_ctrl::get_top_dir;
mod misc;
use log::info;
use looper::Looper;
use misc::logger::init_misc;

fn main() -> anyhow::Result<()> {
    init_misc();
    analysis_cgroup();
    let rs1 = get_top_dir()?;
    let rs2 = get_backend_dir()?;
    info!("{}-{}", rs1, rs2);
    // Looper::new().enter_loop();
    Ok(())
}
