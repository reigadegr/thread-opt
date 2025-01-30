mod process_monitor;
use libc::pid_t;
use process_monitor::ProcessMonitor;

#[derive(Debug)]
pub struct Controller {
    process_monitor: ProcessMonitor,
    max_tid: Option<pid_t>,        // 存储最大负载的线程tid
    second_max_tid: Option<pid_t>, // 新增字段，存储第二大的线程tid
}

impl Controller {
    pub fn new() -> Self {
        Self {
            process_monitor: ProcessMonitor::new(),
            max_tid: None,
            second_max_tid: None, // 初始化second_max_tid
        }
    }

    pub fn init_game(&mut self, pid: pid_t) {
        self.process_monitor.set_pid(Some(pid));
        self.max_tid = None;
        self.second_max_tid = None; // 重置second_max_tid
    }

    pub fn init_default(&mut self) {
        self.process_monitor.set_pid(None);
        self.max_tid = None;
        self.second_max_tid = None; // 重置second_max_tid
    }

    pub fn update_max_usage_tid(&mut self) {
        if let Some((tid1, tid2)) = self.process_monitor.update_max_usage_tid() {
            self.max_tid = Some(tid1);
            self.second_max_tid = Some(tid2);
        }
    }

    // 新增获取tid的方法
    pub const fn first_max_tid(&self) -> Option<pid_t> {
        self.max_tid
    }

    // 新增获取second_max_tid的方法
    pub const fn second_max_tid(&self) -> Option<pid_t> {
        self.second_max_tid
    }
}
