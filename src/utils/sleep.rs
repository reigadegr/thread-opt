use core::time::Duration;
pub fn sleep_millis(micro_seconds: u64) {
    std::thread::sleep(Duration::from_millis(micro_seconds));
}
