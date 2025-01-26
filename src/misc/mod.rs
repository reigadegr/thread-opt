pub mod logger;
use super::misc::logger::{init_log, log_metainfo};
use std::process;

pub fn init_misc() {
    working_in_background();
    let _ = init_log();
    log_metainfo();
}

fn working_in_background() {
    let self_pid = process::id();
    let _ = std::fs::write("/dev/cpuset/background/tasks", self_pid.to_string());
}
