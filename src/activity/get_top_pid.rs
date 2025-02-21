use crate::utils::sleep::sleep_millis;
use atoi::atoi;
use dumpsys_rs::Dumpsys;
use inotify::{Inotify, WatchMask};
use libc::pid_t;
use log::info;

#[derive(Default)]
pub struct TopPidInfo {
    pid: pid_t,
}

impl TopPidInfo {
    pub fn new(dump: &[u8]) -> Self {
        let pid = dump
            .split(|&b| b == b'\n')
            .find(|line| line.windows(4).any(|w| w == b" TOP"))
            .and_then(|line| {
                // 修正为字节切片的处理方式
                line.split(|&b| b.is_ascii_whitespace())
                    .filter_map(|s| (!s.is_empty()).then_some(s))
                    .nth(4)
            })
            .and_then(atoi::<pid_t>)
            .unwrap_or_default();
        #[cfg(debug_assertions)]
        println!("pid为-{pid:?}-");
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
        // #[cfg(debug_assertions)]
        // let start = std::time::Instant::now();
        let dump = loop {
            match self.dumper.dump_to_byte(&["lru"]) {
                Ok(dump) => break dump,
                Err(e) => {
                    info!("Failed to dump windows: {}, retrying", e);
                    sleep_millis(500);
                }
            }
        };
        // #[cfg(debug_assertions)]
        // {
        // let end = start.elapsed();
        // log::debug!("读toppid成时间: {:?}", end);
        // }
        TopPidInfo::new(&dump)
    }
}
