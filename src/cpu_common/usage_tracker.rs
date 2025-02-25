use crate::utils::node_reader::read_to_byte_sp;
use atoi::atoi;
use libc::pid_t;
use stringzilla::sz;
extern crate alloc;
use std::io::Write;

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
    let mut stat_path = [0u8; 32];
    stat_path[0..6].copy_from_slice(&b"/proc/"[..]);
    if write!(&mut stat_path[6..], "{tid}/schedstat").is_err() {
        return 0;
    }

    let buffer = read_to_byte_sp::<32>(&stat_path).unwrap_or([0u8; 32]);

    let pos = sz::find(buffer, b" ");
    let buffer = pos.map_or(&buffer[..], |pos| &buffer[..pos]);
    atoi::<u64>(buffer).unwrap_or(0)
}
