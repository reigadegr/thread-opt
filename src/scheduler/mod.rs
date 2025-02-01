pub mod looper;
use crate::{
    activity::{get_tid_info::TidUtils, get_top_tid::TopAppUtils},
    cpu_common::Controller,
};
use looper::Looper;

pub struct Scheduler {
    looper: Looper,
}

impl Scheduler {
    #[must_use]
    pub fn new() -> Self {
        let top_app_utils = TopAppUtils::new();
        let tid_utils = TidUtils::new();
        let controller = Controller::new();
        Self {
            looper: Looper::new(top_app_utils, tid_utils, controller),
        }
    }

    pub fn start_run(&mut self) {
        self.looper.enter_loop();
    }
}
