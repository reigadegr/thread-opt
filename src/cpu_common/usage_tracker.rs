use atoi::atoi;
use libc::pid_t;
extern crate alloc;
use alloc::format;
use std::{fs::File, io::Read};

#[derive(Debug, Clone)]
pub struct UsageTracker {
    tid: pid_t,
}

impl UsageTracker {
    pub const fn new(tid: pid_t) -> Self {
        Self { tid }
    }

    pub fn try_calculate(&self) -> u64 {
        get_thread_cpu_time(self.tid)
    }
}

fn get_thread_cpu_time(tid: pid_t) -> u64 {
    let stat_path = format!("/proc/{tid}/schedstat");
    let Ok(mut file) = File::open(&stat_path) else {
        return 0;
    };
    let mut buffer = [0u8; 32];
    let Ok(_) = file.read(&mut buffer) else {
        return 0;
    };
    let mut parts = buffer.split(|&b| b == b' ');
    let first_part = parts.next().unwrap_or_default();
    atoi::<u64>(first_part).unwrap_or(0)
}
