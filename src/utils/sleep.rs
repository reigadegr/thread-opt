use std::{thread, time::Duration};

pub fn sleep_millis(milliseconds: u64) {
    thread::sleep(Duration::from_millis(milliseconds));
}
