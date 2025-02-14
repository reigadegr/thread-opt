mod process_monitor;
mod usage_tracker;
use libc::pid_t;
use process_monitor::ProcessMonitor;

#[derive(Debug)]
pub struct Controller {
    process_monitor: ProcessMonitor,
    first_max_tid: pid_t,
    second_max_tid: pid_t,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            process_monitor: ProcessMonitor::new(),
            first_max_tid: -1,
            second_max_tid: -1,
        }
    }

    pub fn init_game(&self, work_state: bool) {
        self.process_monitor.set_work_state(Some(work_state));
    }

    pub fn init_default(&self) {
        self.process_monitor.set_work_state(None);
    }

    pub fn update_max_usage_tid(&mut self) {
        let (tid1, tid2) = self.process_monitor.update_max_usage_tid();
        self.first_max_tid = tid1;
        self.second_max_tid = tid2;
    }

    pub const fn first_max_tid(&self) -> pid_t {
        self.first_max_tid
    }

    pub const fn second_max_tid(&self) -> pid_t {
        self.second_max_tid
    }
}
