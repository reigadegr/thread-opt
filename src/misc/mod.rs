pub mod logger;
use super::misc::logger::{init_log, log_metainfo};
use anyhow::Result;
use libc::pthread_setname_np;
use log::info;
use std::{ffi::CString, process};

pub fn init_misc() {
    working_in_background();
    let rs = set_main_thread_name();
    if rs.is_err() {
        info!("Cannot rename the main thread name.");
    }
    let _ = init_log();
    log_metainfo();
}

fn working_in_background() {
    let self_pid = process::id();
    let _ = std::fs::write("/dev/cpuset/background/tasks", self_pid.to_string());
}

fn set_main_thread_name() -> Result<()> {
    let thread_name = CString::new("affinity_optimizer")?;

    unsafe {
        pthread_setname_np(libc::pthread_self(), thread_name.as_ptr());
    }
    Ok(())
}
