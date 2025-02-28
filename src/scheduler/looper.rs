use crate::{
    activity::{ActivityUtils, get_tid_info::get_process_name},
    policy::pkg_cfg::{CUST_CONFIGS, PACKAGE_CONFIGS, StartArgs},
    utils::{affinity_utils::global_cpu_utils::bind_list_to_background, sleep::sleep_secs},
};
use compact_str::CompactString;
use libc::pid_t;
use log::info;

pub struct Looper {
    activity_utils: ActivityUtils,
    global_package: CompactString,
    pid: pid_t,
}

impl Looper {
    pub fn new(activity_utils: ActivityUtils) -> Self {
        Self {
            activity_utils,
            global_package: CompactString::new(""),
            pid: -1,
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
            sleep_secs(1);
            {
                let pid = self.activity_utils.top_app_utils.get_top_pid();
                if self.pid == pid {
                    continue 'outer;
                }
                self.pid = pid;
                let name = get_process_name(pid).unwrap_or_default();
                self.global_package = name;
            }

            for (package_list, start_task) in CUST_CONFIGS.iter() {
                if self.handle_package_list(package_list, start_task) {
                    continue 'outer;
                }
            }

            for (package_list, start_task) in PACKAGE_CONFIGS {
                if self.handle_package_list(package_list, start_task) {
                    continue 'outer;
                }
            }
        }
    }
}
