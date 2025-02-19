pub mod get_tid_info;
pub mod get_top_pid;

use get_tid_info::TidUtils;
use get_top_pid::TopAppUtils;

pub struct ActivityUtils {
    pub top_app_utils: TopAppUtils,
    pub tid_utils: TidUtils,
}

impl ActivityUtils {
    pub fn new() -> Self {
        Self {
            top_app_utils: TopAppUtils::new(),
            tid_utils: TidUtils::new(),
        }
    }
}
