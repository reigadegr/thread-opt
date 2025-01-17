mod shared;
// use crate::shared::get_top_app::get_top_app_pid_name;
use crate::shared::get_top_app::TopAppUtils;
use crate::shared::logger::init_log;
use log::info;
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
    let mut a = TopAppUtils::new();
    let b = a.init_top_app_pid_name();
    info!("{}--{}", b.pid, b.name);
    // // 打印这两个值
    // info!("pid: -{}-", pid);
    // info!("topappname: -{}-", name);
    Ok(())
}
