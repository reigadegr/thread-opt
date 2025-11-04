use crate::utils::sleep::sleep_secs;
use atoi::atoi;
use dumpsys_rs::Dumpsys;
use inotify::{Inotify, WatchMask};
use likely_stable::LikelyOption;
use log::info;
use ndk_sys::android_get_device_api_level;
use stringzilla::stringzilla::{StringZillableBinary, find};

#[derive(Default)]
pub struct TopPidInfo {
    pid: i32,
}

impl TopPidInfo {
    pub fn new(dump: &[u8], android_version: u8) -> Self {
        let pid = if android_version == 16 {
            Self::parse_a16(dump)
        } else {
            Self::parse_a15(dump)
        };
        Self { pid }
    }

    fn parse_a15(dump: &[u8]) -> i32 {
        let multi_window = find(dump, b"Window #1").is_some();

        let pid = if multi_window {
            dump.sz_rsplits(&b"\n")
                .find(|line| find(line, b"Session{").is_some())
        } else {
            dump.sz_splits(&b"\n")
                .find(|line| find(line, b"Session{").is_some())
        };

        pid.and_then_likely(|line| {
            line.sz_rfind(b":")
                .and_then_likely(|pos1| line[..pos1].sz_rfind(b" ").map(|pos2| &line[pos2 + 1..]))
        })
        .and_then_likely(atoi::<i32>)
        .unwrap_or_default()
    }

    fn parse_a16(dump: &[u8]) -> i32 {
        let multi_window = find(dump, b"Window #2").is_some();

        let pid = if multi_window {
            dump.sz_rsplits(&b"\n")
                .filter(|line| find(line, b"Session{").is_some())
                .nth(1)
        } else {
            dump.sz_splits(&b"\n")
                .find(|line| find(line, b"Session{").is_some())
        };

        pid.and_then_likely(|line| {
            line.sz_rfind(b":")
                .and_then_likely(|pos1| line[..pos1].sz_rfind(b" ").map(|pos2| &line[pos2 + 1..]))
        })
        .and_then_likely(atoi::<i32>)
        .unwrap_or_default()
    }
}

pub struct TopAppUtils {
    android_version: u8,
    dumper: Dumpsys,
    inotify: Inotify,
}

impl TopAppUtils {
    fn init_dumper() -> Dumpsys {
        loop {
            match Dumpsys::new("window") {
                Some(d) => break d,
                None => sleep_secs(1),
            }
        }
    }

    fn init_inotify() -> Inotify {
        let inotify = Inotify::init().unwrap();
        inotify
            .watches()
            .add("/dev/input", WatchMask::ACCESS)
            .unwrap();
        inotify
    }

    fn get_android_version() -> u8 {
        let api = unsafe { android_get_device_api_level() };
        match api {
            36 => 16,
            _ => 15,
        }
    }

    pub fn new() -> Self {
        let android_version = Self::get_android_version();
        let dumper = Self::init_dumper();
        let inotify = Self::init_inotify();

        Self {
            android_version,
            dumper,
            inotify,
        }
    }

    pub fn get_top_pid(&mut self) -> i32 {
        self.set_top_pid().pid
    }

    pub fn set_top_pid(&mut self) -> TopPidInfo {
        loop {
            match self.inotify.read_events_blocking(&mut [0; 1024]) {
                Ok(_) => break,
                Err(e) => {
                    info!("Failed to read events: {e}, retrying");
                    sleep_secs(1);
                }
            }
        }
        let dump = loop {
            match self.dumper.dump_to_byte::<65536>(&["visible-apps"]) {
                Ok(dump) => break dump,
                Err(e) => {
                    info!("Failed to dump windows: {e}, retrying");
                    sleep_secs(1);
                }
            }
        };
        TopPidInfo::new(&dump, self.android_version)
    }
}
