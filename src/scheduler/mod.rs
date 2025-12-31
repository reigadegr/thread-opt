pub mod looper;
pub mod name_match;
use crate::{activity::ActivityUtils, config::AtomicConfig};
use inotify::{Inotify, WatchMask};
use log::{error, info};
use looper::Looper;
pub mod usage_top1;
pub mod usage_top2;

pub struct Scheduler {
    looper: Looper,
    atomic_config: AtomicConfig,
}

impl Scheduler {
    #[must_use]
    pub fn new() -> Self {
        let atomic_config = AtomicConfig::init();
        let activity_utils = ActivityUtils::new();

        let looper = Looper::new(activity_utils);

        Self {
            looper,
            atomic_config,
        }
    }

    fn start_config_watcher(&self) {
        let config_ptr = &raw const self.atomic_config as usize;

        std::thread::spawn(move || {
            let config_ref = unsafe { &*(config_ptr as *const AtomicConfig) };

            let config_path = "/data/adb/modules/thread_opt/thread_opt.toml";

            let mut inotify = match Inotify::init() {
                Ok(i) => i,
                Err(e) => {
                    error!("Failed to initialize inotify for config watcher: {e}");
                    return;
                }
            };

            if let Err(e) = inotify.watches().add(config_path, WatchMask::CLOSE_WRITE) {
                error!("Failed to add watch for config file: {e}");
                return;
            }

            info!("Config watcher started");

            loop {
                match inotify.read_events_blocking(&mut [0; 1024]) {
                    Ok(_events) => {
                        std::thread::sleep(std::time::Duration::from_millis(50));
                        config_ref.reload();
                    }
                    Err(e) => {
                        error!("Failed to read inotify events: {e}");
                        std::thread::sleep(std::time::Duration::from_secs(1));
                    }
                }
            }
        });
    }

    pub fn start_run(&mut self) {
        self.start_config_watcher();
        self.looper.enter_loop(&self.atomic_config);
    }
}
