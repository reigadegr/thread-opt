use crate::{
    activity::{get_tid_info::get_process_name, ActivityUtils},
    policy::pkg_cfg::{StartArgs, PACKAGE_CONFIGS},
    utils::global_cpu_utils::bind_list_to_background,
};
use compact_str::CompactString;
use libc::pid_t;
use log::info;
use std::time::Duration;

pub struct Looper {
    pid: pid_t,
    global_package: CompactString,
    activity_utils: ActivityUtils,
}

impl Looper {
    pub fn new(activity_utils: ActivityUtils) -> Self {
        Self {
            pid: -1,
            global_package: CompactString::new(""),
            activity_utils,
        }
    }

    fn game_exit(&mut self) {
        info!("Exiting game\n");
        let tid_list = self.activity_utils.tid_utils.get_tid_list(self.pid);
        bind_list_to_background(tid_list);
        self.pid = -1;
        self.activity_utils.tid_utils.tid_info.task_map.clear();
        self.activity_utils.tid_utils.tid_info.tid_list.clear();
    }

    fn start_bind_common<F>(&mut self, start_task: F)
    where
        F: Fn(&mut StartArgs),
    {
        start_task(&mut StartArgs {
            activity_utils: &mut self.activity_utils,
            pid: self.pid,
        });
        self.game_exit();
    }

    fn handle_package_list<F>(&mut self, package_list: &[&str], start_task: F) -> bool
    where
        F: Fn(&mut StartArgs),
    {
        for &package in package_list {
            if package == self.global_package {
                info!("Detected target App: {}", self.global_package);
                self.start_bind_common(start_task);
                return true;
            }
        }
        false
    }

    pub fn enter_loop(&mut self) {
        'outer: loop {
            std::thread::sleep(Duration::from_millis(1000));
            {
                let pid = self.activity_utils.top_app_utils.get_pid();
                if self.pid == pid {
                    continue 'outer;
                }
                self.pid = pid;
                let name = get_process_name(pid).unwrap_or_default();
                self.global_package = name;
            }
            for (package_list, start_task) in PACKAGE_CONFIGS.iter() {
                if self.handle_package_list(package_list, start_task) {
                    continue 'outer;
                }
            }
        }
    }
}
