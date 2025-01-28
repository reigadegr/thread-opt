mod process_monitor;
use process_monitor::ProcessMonitor;

#[derive(Debug)]
pub struct Controller {
    process_monitor: ProcessMonitor,
    util_max: Option<f64>,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            process_monitor: ProcessMonitor::new(),
            util_max: None,
        }
    }

    pub fn init_game(&mut self, pid: i32) {
        self.process_monitor.set_pid(Some(pid));
        self.util_max = None;
    }

    pub fn init_default(&mut self) {
        self.process_monitor.set_pid(None);
        self.util_max = None;
    }

    fn update_util_max(&mut self) {
        if let Some(util_max) = self.process_monitor.update_util_max() {
            self.util_max = Some(util_max);
        }
    }

    pub fn util_max(&self) -> f64 {
        self.util_max.unwrap_or_default()
    }
}
