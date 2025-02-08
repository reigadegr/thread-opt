use dumpsys_rs::Dumpsys;
use libc::pid_t;
use likely_stable::unlikely;
#[cfg(debug_assertions)]
use log::debug;
use log::info;
use std::time::{Duration, Instant};

#[derive(Default)]
pub struct TopPidInfo {
    pid: pid_t,
}

impl TopPidInfo {
    pub fn new(dump: &str) -> Self {
        if unlikely(!dump.contains(" TOP")) {
            return Self::default();
        }

        let dump: pid_t = dump
            .lines()
            .filter(|l| l.contains(" TOP"))
            .take(1)
            .filter_map(|l| l.split_whitespace().nth(4))
            .filter_map(|l| l.split('/').next())
            .filter_map(|s| s.split(':').next())
            .map(|p| p.trim().parse().unwrap_or_default())
            .next()
            .unwrap_or_default();
        Self { pid: dump }
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
        #[cfg(debug_assertions)]
        let start = std::time::Instant::now();
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
        #[cfg(debug_assertions)]
        {
            let end = start.elapsed();
            debug!("读取包名时间: {:?}", end);
        }
        self.last_refresh = Instant::now();
        &self.activity_info
    }
}
