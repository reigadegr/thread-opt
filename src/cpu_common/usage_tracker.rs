use atoi::atoi;
use libc::pid_t;
extern crate alloc;
use alloc::{format, vec::Vec};

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
    let stat_content = std::fs::read(stat_path).unwrap_or_else(|_| Vec::new());
    let mut parts = stat_content.split(|&b| b == b' ');
    let first_part = parts.next().unwrap_or_default();
    atoi::<u64>(first_part).unwrap_or(0)
}
