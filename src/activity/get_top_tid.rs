use dumpsys_rs::Dumpsys;
use libc::pid_t;
use log::info;
use std::time::{Duration, Instant};

#[derive(Default)]
pub struct TopPidInfo {
    pid: pid_t,
}

impl TopPidInfo {
    pub fn new(dump: &str) -> Self {
        if !dump.contains(" TOP") {
            return Self::default();
        }

        let dump = dump
            .lines()
            .filter(|l| l.contains(" TOP"))
            .filter(|l| l.contains(": fg"))
            .filter_map(|l| l.split_whitespace().nth(4))
            .filter_map(|l| l.split('/').next())
            .collect::<Vec<_>>()
            .join("");

        let pid = dump.split(':').next().unwrap_or("0");

        Self {
            pid: pid.parse::<pid_t>().unwrap_or_default(),
        }
    }
}

pub struct TopAppUtils {
    dumper: Dumpsys,
    activity_info: TopPidInfo,
    last_refresh: Instant,
}

impl TopAppUtils {
    pub fn new() -> Self {
        let dumper = loop {
            match Dumpsys::new("activity") {
                Some(d) => break d,
                None => std::thread::sleep(Duration::from_millis(100)),
            }
        };
        Self {
            dumper,
            activity_info: TopPidInfo::default(),
            last_refresh: Instant::now(),
        }
    }

    pub fn get_pid(&mut self) -> &pid_t {
        &self.set_top_app_pid_name().pid
    }

    pub fn set_top_app_pid_name(&mut self) -> &TopPidInfo {
        if self.last_refresh.elapsed() < Duration::from_millis(1000) {
            return &self.activity_info;
        }

        let dump = loop {
            match self.dumper.dump(&["lru"]) {
                Ok(dump) => break dump,
                Err(e) => {
                    info!("Failed to dump windows: {}, retrying", e);
                    std::thread::sleep(Duration::from_millis(100));
                }
            }
        };
        self.activity_info = TopPidInfo::new(&dump);
        self.last_refresh = Instant::now();
        &self.activity_info
    }
}
