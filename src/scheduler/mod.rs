pub mod looper;
use crate::activity::ActivityUtils;
use crate::cpu_common::Controller;
use looper::Looper;

pub struct Scheduler {
    looper: Looper,
}

impl Scheduler {
    #[must_use]
    pub fn new() -> Self {
        let activity_utils = ActivityUtils::new();
        let controller = Controller::new();
        Self {
            looper: Looper::new(activity_utils, controller),
        }
    }

    pub fn start_run(&mut self) {
        self.looper.enter_loop();
    }
}
