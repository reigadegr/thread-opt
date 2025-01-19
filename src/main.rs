mod activity;
mod affinity_set;
mod fs_utils;
mod looper;
use crate::fs_utils::analysis_cgroup;
use crate::fs_utils::dir_ctrl::get_backend_dir;
use crate::fs_utils::dir_ctrl::get_top_dir;
use crate::fs_utils::dir_ctrl::middle_dir_ctrl;
mod misc;
use log::info;
use looper::Looper;
use misc::logger::init_misc;

fn main() -> anyhow::Result<()> {
    init_misc();
    analysis_cgroup();
    let rs = middle_dir_ctrl();
    if rs.is_err() {
        info!("出错");
    } else {
        info!("没出");
    }
    let rs1 = get_top_dir()?;
    let rs2 = get_backend_dir()?;
    info!("{}-{}", rs1, rs2);
    // Looper::new().enter_loop();
    Ok(())
}
