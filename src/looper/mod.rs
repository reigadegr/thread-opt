use super::activity::get_top_app::TopAppUtils;
use crate::activity::get_tid_info::TidUtils;
use log::info;
use std::time::Duration;
pub struct Looper {
    windows_info: TopAppUtils,
    tid_utils: TidUtils,
}

impl Looper {
    pub fn new() -> Self {
        Self {
            windows_info: TopAppUtils::new(),
            tid_utils: TidUtils::new(),
        }
    }

    pub fn enter_loop(&mut self) -> anyhow::Result<()> {
        loop {
            let name = self.windows_info.get_top_app();
            info!("{}", name);
            std::thread::sleep(Duration::from_millis(500));
            let pid = self.windows_info.get_pid();
            let tids = self.tid_utils.get_task_map(pid);
            info!("{:?}", tids);

            let tl2 = self.tid_utils.get_tid_list(pid);
            info!("{:?}", tl2);
        }
    }
}
