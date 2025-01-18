mod activity;
mod looper;
mod misc;
// use activity::get_top_app::TopAppUtils;
use log::info;
use looper::Looper;
use misc::logger::init_log;
use std::fs;
use std::process;
fn init_misc() {
    let _ = init_log();
    let self_pid = process::id();
    let _ = fs::write("/dev/cpuset/background/tasks", self_pid.to_string());
}

fn main() -> anyhow::Result<()> {
    init_misc();
    info!("Hello, world!");
    Looper::new().enter_loop();
    Ok(())
}
