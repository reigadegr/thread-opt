use dumpsys_rs::Dumpsys;
use log::info;
use std::time::Duration;

#[derive(Default)]
pub struct ActivityInfo {
    pub pid: i32,
    pub name: String,
}

impl ActivityInfo {
    pub fn new(dump: &str) -> Self {
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
    pub dumper: Dumpsys,
    pub activity_info: ActivityInfo,
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
        }
    }

    pub fn init_top_app_pid_name(&mut self) -> &ActivityInfo {
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
        &self.activity_info
    }
}
