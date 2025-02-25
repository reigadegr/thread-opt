pub mod logger;
use crate::{cgroup::group_info::print_group_core, utils::node_reader::write_to_byte};
use anyhow::Result;
use libc::getpid;
use likely_stable::unlikely;
use log::info;
use logger::{init_log, log_metainfo};
extern crate alloc;
use alloc::{ffi::CString, string::ToString};

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
    unsafe {
        let self_pid = getpid().to_string();
        let _ = write_to_byte::<6>("/dev/cpuset/background/tasks", &self_pid);
    }
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
    info!("开源地址: https://github.com/reigadegr/thread-opt");
}
