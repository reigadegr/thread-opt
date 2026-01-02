pub mod looper;
pub mod name_match;
pub mod usage_top1;
pub mod usage_top2;

use crate::{activity::ActivityUtils, config::AtomicConfig, utils::sleep::sleep_millis};
use inotify::{Inotify, WatchMask};
use log::{error, info};
use looper::Looper;
use std::sync::Arc;

pub struct Scheduler {
    looper: Looper,
    atomic_config: Arc<AtomicConfig>,
}

impl Scheduler {
    #[must_use]
    pub fn new() -> Self {
        let atomic_config = Arc::new(AtomicConfig::init());
        let activity_utils = ActivityUtils::new();
        let looper = Looper::new(activity_utils);

        Self {
            looper,
            atomic_config,
        }
    }

    fn start_config_watcher(&self) {
        let config = Arc::clone(&self.atomic_config);

        std::thread::spawn(move || {
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
                    Ok(_) => {
                        sleep_millis(50);
                        config.reload();
                    }
                    Err(e) => {
                        error!("Failed to read inotify events: {e}");
                        sleep_millis(1000);
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
