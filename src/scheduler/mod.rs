pub mod looper;
use crate::{
    activity::{get_tid_info::TidUtils, get_top_tid::TopAppUtils},
    cpu_common::Controller,
};
use looper::Looper;

pub struct Scheduler<'a> {
    looper: Looper<'a>,
}

impl<'a> Scheduler<'a> {
    #[must_use]
    pub fn new(top_app_utils: &'a mut TopAppUtils) -> Self {
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
