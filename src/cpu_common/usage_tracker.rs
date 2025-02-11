use atoi::atoi;
use libc::pid_t;

#[cfg(debug_assertions)]
use log::debug;
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
    #[cfg(debug_assertions)]
    let start = std::time::Instant::now();
    let stat_content = std::fs::read(stat_path).unwrap_or_else(|_| Vec::new());
    let mut parts = stat_content.split(|b| *b == b' ');
    let first_part = parts.next().unwrap_or_default();
    let rs = atoi::<u64>(first_part).unwrap_or(0);
    #[cfg(debug_assertions)]
    {
        let end = start.elapsed();
        debug!("读取+计算负载完成时间: {:?}", end,);
    }
    rs
}

// fn get_thread_cpu_time(tid: i32) -> u64 {
// let stat_path = format!("/proc/{tid}/schedstat");
// #[cfg(debug_assertions)]
// let start = std::time::Instant::now();
// let stat_content = std::fs::read_to_string(stat_path).unwrap_or_else(|_| String::from("0"));
// let parts = stat_content.split_whitespace().next().unwrap_or_default();
// let time = parts.parse::<u64>().unwrap_or(0);
// #[cfg(debug_assertions)]
// {
// let end = start.elapsed();
// debug!("读取+计算负载完成时间: {:?}", end,);
// }
// time
// }
