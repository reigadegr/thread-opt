pub mod logger;
use crate::misc::logger::{init_log, log_metainfo};
use crate::{cgroup::group_info::get_background_group, utils::affinity_setter::bind_thread_to_cpu};
use libc::pid_t;
use std::process;

pub fn init_misc() {
    let _ = init_log();
    log_metainfo();
}

pub fn working_in_background() {
    let self_pid: pid_t = match process::id().try_into() {
        Ok(pid) => pid,
        Err(_) => return,
    };
    bind_thread_to_cpu(get_background_group(), self_pid);
}
