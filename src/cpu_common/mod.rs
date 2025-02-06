mod process_monitor;
use libc::pid_t;
use process_monitor::ProcessMonitor;

#[derive(Debug)]
pub struct Controller {
    process_monitor: ProcessMonitor,
    max_tid: Option<pid_t>,
    second_max_tid: Option<pid_t>,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            process_monitor: ProcessMonitor::new(),
            max_tid: None,
            second_max_tid: None,
        }
    }

    pub fn init_game(&mut self, pid: bool) {
        self.process_monitor.set_pid(Some(pid));
        self.max_tid = None;
        self.second_max_tid = None;
    }

    pub fn init_default(&mut self) {
        self.process_monitor.set_pid(None);
        self.max_tid = None;
        self.second_max_tid = None;
    }

    pub fn update_max_usage_tid(&mut self) {
        if let Some((tid1, tid2)) = self.process_monitor.update_max_usage_tid() {
            self.max_tid = Some(tid1);
            self.second_max_tid = Some(tid2);
        }
    }

    pub const fn first_max_tid(&self) -> Option<pid_t> {
        self.max_tid
    }

    pub const fn second_max_tid(&self) -> Option<pid_t> {
        self.second_max_tid
    }
}
