pub mod looper;
use crate::activity::ActivityUtils;
use looper::Looper;

pub struct Scheduler {
    looper: Looper,
}

impl Scheduler {
    #[must_use]
    pub fn new() -> Self {
        let activity_utils = ActivityUtils::new();
        Self {
            looper: Looper::new(activity_utils),
        }
    }

    pub fn start_run(&mut self) {
        self.looper.enter_loop();
    }
}
