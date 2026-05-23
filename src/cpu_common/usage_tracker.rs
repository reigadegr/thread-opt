use atoi::atoi;
use memchr::memchr;

use crate::utils::node_reader::{get_proc_path, read_to_byte};

#[derive(Debug, Clone)]
pub struct UsageTracker {
    tid: i32,
}

impl UsageTracker {
    pub const fn new(tid: i32) -> Self {
        Self { tid }
    }

    pub fn try_calculate(&self) -> u64 {
        get_thread_cpu_time(self.tid)
    }
}

fn get_thread_cpu_time(tid: i32) -> u64 {
    let stat_path = get_proc_path::<32>(tid, b"/schedstat");
    let Ok(buffer) = read_to_byte::<32>(&stat_path) else {
        return 0;
    };

    let pos = memchr(b' ', &buffer);
    let buffer = match pos {
        Some(pos) => &buffer[..pos],
        None => &buffer[..],
    };
    let Some(usage) = atoi::<u64>(buffer) else {
        return 0;
    };
    usage
}
