use crate::utils::node_reader::read_to_byte;
use atoi::atoi;
use itoa::Buffer;
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
    let mut stat_path = [0u8; 32];
    stat_path[0..6].copy_from_slice(b"/proc/");

    let mut itoa_buf = Buffer::new();
    let tid_byte = itoa_buf.format(tid).as_bytes();

    let sched_part = b"/schedstat";
    let mid = 6 + tid_byte.len();
    let end = mid + 10;

    stat_path[6..mid].copy_from_slice(tid_byte);
    stat_path[mid..end].copy_from_slice(sched_part);

    let buffer = read_to_byte::<32>(&stat_path).unwrap_or([0u8; 32]);

    let pos = sz::find(buffer, b" ");
    let buffer = pos.map_or(&buffer[..], |pos| &buffer[..pos]);
    atoi::<u64>(buffer).unwrap_or(0)
}
