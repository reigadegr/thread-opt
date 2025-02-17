use core::time::Duration;
use std::thread::sleep;

pub fn sleep_millis(millis_second: u64) {
    sleep(Duration::from_millis(millis_second));
}
