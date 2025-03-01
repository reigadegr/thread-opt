use crate::utils::node_reader::{get_proc_path, read_to_byte};
use atoi::atoi;
use libc::pid_t;
use stringzilla::sz;

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
    let stat_path = get_proc_path::<32, 10>(tid, b"/schedstat");
    let buffer = read_to_byte::<32>(&stat_path).unwrap_or([0u8; 32]);

    let pos = sz::find(buffer, b" ");
    let buffer = pos.map_or(&buffer[..], |pos| &buffer[..pos]);
    atoi::<u64>(buffer).unwrap_or(0)
}
