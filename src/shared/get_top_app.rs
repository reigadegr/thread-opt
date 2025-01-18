use dumpsys_rs::Dumpsys;
use log::info;
use std::time::{Duration, Instant};
#[derive(Default)]
pub struct ActivityInfo {
    pub pid: i32,
    pub name: String,
}

impl ActivityInfo {
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
            .collect::<String>();

        let parts: Vec<&str> = dump.split(':').collect();
        if parts.len() != 2 {
            return ActivityInfo::default();
        }
        Self {
            pid: parts[0].parse::<i32>().unwrap_or_default(),
            name: parts[1].to_string(),
        }
    }
}

pub struct TopAppUtils {
    dumper: Dumpsys,
    activity_info: ActivityInfo,
    last_refresh: Instant,
}

impl TopAppUtils {
    pub fn new() -> Self {
        let dumper = loop {
            match Dumpsys::new("activity") {
                Some(d) => break d,
                None => std::thread::sleep(Duration::from_millis(1000)),
            }
        };
        Self {
            dumper,
            activity_info: ActivityInfo::default(),
            last_refresh: Instant::now(),
        }
    }

    pub fn get_pid(&mut self) -> &i32 {
        &self.set_top_app_pid_name().pid
    }

    pub fn get_top_app(&mut self) -> &str {
        &self.set_top_app_pid_name().name
    }

    pub fn set_top_app_pid_name(&mut self) -> &ActivityInfo {
        if self.last_refresh.elapsed() < Duration::from_millis(1000) {
            return &self.activity_info;
        }

        let dump = loop {
            match self.dumper.dump(&["lru"]) {
                Ok(dump) => break dump,
                Err(e) => {
                    info!("Failed to dump windows: {}, retrying", e);
                    std::thread::sleep(Duration::from_millis(1000));
                }
            }
        };
        self.activity_info = ActivityInfo::new(&dump);
        self.last_refresh = Instant::now();
        &self.activity_info
    }
}
