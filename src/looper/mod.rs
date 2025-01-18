use super::activity::get_top_app::TopAppUtils;
use crate::activity::get_tid_info::TidUtils;
use log::info;
use std::time::Duration;
pub struct Looper {
    top_app_utils: TopAppUtils,
    tid_utils: TidUtils,
}

impl Looper {
    pub fn new() -> Self {
        Self {
            top_app_utils: TopAppUtils::new(),
            tid_utils: TidUtils::new(),
        }
    }

    pub fn enter_loop(&mut self) {
        loop {
            let name = self.top_app_utils.get_top_app();
            info!("{}", name);

            let pid = self.top_app_utils.get_pid();
            let tids = self.tid_utils.get_task_map(pid);
            info!("{:?}", tids);

            let tl2 = self.tid_utils.get_tid_list(pid);
            info!("{:?}", tl2);

            std::thread::sleep(Duration::from_millis(500));
        }
    }
}
