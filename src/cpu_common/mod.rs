mod process_monitor;
use libc::pid_t;
use process_monitor::ProcessMonitor;

#[derive(Debug)]
pub struct Controller {
    process_monitor: ProcessMonitor,
    util_max: Option<f64>,
    max_tid: Option<pid_t>, // 新增字段存储tid
}

impl Controller {
    pub fn new() -> Self {
        Self {
            process_monitor: ProcessMonitor::new(),
            util_max: None,
            max_tid: None,
        }
    }

    pub fn init_game(&mut self, pid: pid_t) {
        self.process_monitor.set_pid(Some(pid));
        self.util_max = None;
    }

    pub fn init_default(&mut self) {
        self.process_monitor.set_pid(None);
        self.util_max = None;
    }

    pub fn update_util_max(&mut self) {
        if let Some((util, tid)) = self.process_monitor.update_util_max() {
            self.util_max = Some(util);
            self.max_tid = Some(tid);
        }
    }

    // 新增获取tid的方法
    pub const fn max_tid(&self) -> Option<pid_t> {
        self.max_tid
    }

    pub fn util_max(&self) -> f64 {
        self.util_max.unwrap_or_default()
    }
}
