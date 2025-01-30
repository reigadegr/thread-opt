pub mod looper;
use crate::Controller;
use looper::Looper;

pub struct Scheduler {
    looper: Looper,
}

impl Scheduler {
    #[must_use]
    pub fn new(controller: Controller) -> Self {
        Self {
            looper: Looper::new(controller),
        }
    }

    pub fn start_run(&mut self) {
        self.looper.enter_loop();
    }
}
