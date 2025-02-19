use crate::utils::sleep::sleep_millis;
use dumpsys_rs::Dumpsys;
use inotify::{Inotify, WatchMask};
use libc::pid_t;
use log::info;

#[derive(Default)]
pub struct TopPidInfo {
    pid: pid_t,
}

impl TopPidInfo {
    pub fn new(dump: &str) -> Self {
        let dump: pid_t = dump
            .lines()
            .find(|l| l.contains(" TOP"))
            .and_then(|line| line.split_whitespace().nth(4))
            .and_then(|pid_part| pid_part.split(':').next())
            .and_then(|pid_str| pid_str.parse::<pid_t>().ok())
            .unwrap_or_default();
        Self { pid: dump }
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
                None => sleep_millis(500),
            }
        };
        Self { dumper, inotify }
    }

    pub fn get_top_pid(&mut self) -> pid_t {
        self.set_top_pid().pid
    }

    pub fn set_top_pid(&mut self) -> TopPidInfo {
        self.inotify.read_events_blocking(&mut [0; 1024]).unwrap();
        let dump = loop {
            match self.dumper.dump(&["lru"]) {
                Ok(dump) => break dump,
                Err(e) => {
                    info!("Failed to dump windows: {}, retrying", e);
                    sleep_millis(500);
                }
            }
        };
        TopPidInfo::new(&dump)
    }
}
