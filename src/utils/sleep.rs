use libc::{useconds_t, usleep};

pub fn sleep_micro(micro_seconds: useconds_t) {
    unsafe {
        let _ = usleep(micro_seconds);
    }
}
