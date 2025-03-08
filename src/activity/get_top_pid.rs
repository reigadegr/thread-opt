use crate::utils::sleep::sleep_secs;
use atoi::atoi;
use dumpsys_rs::Dumpsys;
use inotify::{Inotify, WatchMask};
use libc::pid_t;
use likely_stable::LikelyOption;
use log::info;
use stringzilla::{StringZilla, sz};

#[derive(Default)]
pub struct TopPidInfo {
    pid: pid_t,
}

impl TopPidInfo {
    pub fn new(dump: &[u8]) -> Self {
        let pid = dump
            .sz_splits(&b"\n")
            .find(|line| sz::find(line, b" TOP").is_some())
            .and_then_likely(|line| {
                line.sz_rfind(b"/").and_then_likely(|pos1| {
                    line[..pos1].sz_rfind(b" ").map(|pos2| &line[pos2 + 1..])
                })
            })
            .and_then_likely(atoi::<pid_t>)
            .unwrap_or_default();
        Self { pid }
    }
}

pub struct TopAppUtils {
    dumper: Dumpsys,
    inotify: Inotify,
}

impl TopAppUtils {
    pub fn new() -> Self {
        let inotify = Inotify::init().unwrap();
        inotify
            .watches()
            .add("/dev/input", WatchMask::ACCESS)
            .unwrap();

        let dumper = loop {
            match Dumpsys::new("activity") {
                Some(d) => break d,
                None => sleep_secs(1),
            }
        };
        Self { dumper, inotify }
    }

    pub fn get_top_pid(&mut self) -> pid_t {
        self.set_top_pid().pid
    }

    pub fn set_top_pid(&mut self) -> TopPidInfo {
        unsafe {
            self.inotify
                .read_events_blocking(&mut [0; 1024])
                .unwrap_unchecked();
        }
        let dump = loop {
            match self.dumper.dump_to_byte::<1024>(&["lru"]) {
                Ok(dump) => break dump,
                Err(e) => {
                    info!("Failed to dump windows: {}, retrying", e);
                    sleep_secs(1);
                }
            }
        };

        TopPidInfo::new(&dump)
    }
}
