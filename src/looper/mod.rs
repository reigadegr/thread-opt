use super::activity::get_tid_info::get_task_map;
use super::activity::get_top_app::TopAppUtils;
use log::info;
use std::time::Duration;
pub struct Looper {
    windows_info: TopAppUtils,
}

impl Looper {
    pub fn new() -> Self {
        Self {
            windows_info: TopAppUtils::new(),
        }
    }

    pub fn enter_loop(&mut self) -> anyhow::Result<()> {
        loop {
            let name = self.windows_info.get_top_app();
            info!("{}", name);
            std::thread::sleep(Duration::from_millis(500));
            let pid = self.windows_info.get_pid();
            let tids = get_task_map(pid.to_string().as_str());
            info!("{:?}", tids);
        }
    }
}
