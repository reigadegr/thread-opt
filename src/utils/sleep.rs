use libc::sleep;

pub fn sleep_secs(millis_second: u32) {
    unsafe {
        sleep(millis_second);
    }
}
