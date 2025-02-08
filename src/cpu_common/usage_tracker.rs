use libc::pid_t;
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
    let stat_content = std::fs::read_to_string(stat_path).unwrap_or_else(|_| String::from("0"));
    let parts: &str = stat_content.split_whitespace().next().unwrap_or_default();
    parts.parse::<u64>().unwrap_or(0)
}
