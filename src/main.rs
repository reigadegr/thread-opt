mod shared;
// use crate::shared::get_top_app::get_top_app_pid_name;
use crate::shared::get_top_app::TopAppUtils;
use crate::shared::logger::init_log;
use log::info;
use std::cell::RefCell;
use std::fs;
use std::process;
use std::time::Duration;

fn init_misc() {
    let _ = init_log();
    let self_pid = process::id();
    let _ = fs::write("/dev/cpuset/background/tasks", self_pid.to_string());
}

fn main() -> anyhow::Result<()> {
    init_misc();
    info!("Hello, world!");
    let mut b = TopAppUtils::new();
    // let b = b.init_top_app_pid_name();
    // info!("第一次{}--{}", b.pid, b.name);

    loop {
        let c = b.set_top_app_pid_name();
        // info!("第一次{}--{}", b.pid, b.name);
        let top_app = b.get_top_app();
        // let pid = b.get_pid();
        info!("{}--", top_app);
        std::thread::sleep(Duration::from_millis(500));
    }

    // // 打印这两个值
    // info!("pid: -{}-", pid);
    // info!("topappname: -{}-", name);
    Ok(())
}
