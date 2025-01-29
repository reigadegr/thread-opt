pub mod looper;
use looper::Looper;

pub struct Scheduler {
    looper: Looper,
}

impl Scheduler {
    #[must_use]
    pub fn new() -> Self {
        Self {
            looper: Looper::new(),
        }
    }

    pub fn start_run(&mut self) {
        self.looper.enter_loop();
    }
}
