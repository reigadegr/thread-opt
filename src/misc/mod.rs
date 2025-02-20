pub mod logger;
use crate::cgroup::group_info::print_group_core;
use anyhow::Result;
use likely_stable::unlikely;
use log::info;
use logger::{init_log, log_metainfo};
extern crate alloc;
use alloc::ffi::CString;

pub fn init_misc() {
    working_in_background();
    init_log();
    let rs = set_main_thread_name("AffinitySetter");
    if unlikely(rs.is_err()) {
        info!("Cannot rename the main thread name.");
    }
    log_metainfo();
    print_misc();
    print_group_core();
}

fn working_in_background() {
    let self_pid = std::process::id();
    let _ = std::fs::write("/dev/cpuset/background/tasks", self_pid.to_string());
}

fn set_main_thread_name(name: &str) -> Result<()> {
    let truncated_name = if unlikely(name.len() > 15) {
        &name[..15]
    } else {
        name
    };

    let thread_name = CString::new(truncated_name)?;
    unsafe {
        libc::pthread_setname_np(libc::pthread_self(), thread_name.as_ptr());
    }
    Ok(())
}

fn print_misc() {
    info!("免费软件，禁止商用");
    info!("Free software, not for commercial use.");
}
