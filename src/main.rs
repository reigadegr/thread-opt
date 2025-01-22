mod activity;
mod affinity_policy;
mod fs_utils;
mod looper;
mod misc;
use fs_utils::init_working_directory;
use looper::Looper;
use misc::logger::init_misc;

fn main() -> anyhow::Result<()> {
    init_misc();
    let _ = init_working_directory();
    Looper::new().enter_loop();
    Ok(())
}
