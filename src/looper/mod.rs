use super::{
    activity::{
        get_tid_info::{get_process_name, TidUtils},
        get_top_tid::TopAppUtils,
    },
    cgroup::group_info::get_background_group,
    policy::pkg_cfg::PACKAGE_CONFIGS,
    utils::affinity_setter::bind_tid_list_to_cgroup,
};
use hashbrown::HashMap;
use libc::pid_t;
use log::info;
use std::time::Duration;

pub struct Looper {
    pid: pid_t,
    global_package: String,
    top_app_utils: TopAppUtils,
    tid_utils: TidUtils,
}

impl Looper {
    pub fn new() -> Self {
        Self {
            pid: 0,
            global_package: String::new(),
            top_app_utils: TopAppUtils::new(),
            tid_utils: TidUtils::new(),
        }
    }

    fn start_bind_common<F>(&mut self, start_task: F)
    where
        // 传入函数的签名
        F: Fn(&HashMap<pid_t, String>),
    {
        loop {
            let pid = self.top_app_utils.get_pid();
            if pid != &self.pid {
                info!("Exiting game");
                let tid_list = self.tid_utils.get_tid_list(self.pid);
                bind_tid_list_to_cgroup(get_background_group(), tid_list);
                return;
            }
            let task_map = self.tid_utils.get_task_map(*pid);
            start_task(task_map);
            std::thread::sleep(Duration::from_millis(2000));
        }
    }

    fn handle_package_list<F>(&mut self, package_list: &[&str], start_task: F) -> bool
    where
        F: Fn(&HashMap<pid_t, String>),
    {
        for &package in package_list {
            if package == self.global_package {
                info!("Detected target App: {}", self.global_package);
                self.pid = *self.top_app_utils.get_pid();
                self.start_bind_common(start_task);
                return true;
            }
        }
        false
    }

    pub fn enter_loop(&mut self) {
        'outer: loop {
            {
                let pid = self.top_app_utils.get_pid();
                if self.pid == *pid {
                    std::thread::sleep(Duration::from_millis(1000));
                    continue 'outer;
                }
                self.pid = *pid;
                let name = get_process_name(*pid).unwrap_or_default();
                self.global_package = name;
            }
            for (package_list, start_task) in PACKAGE_CONFIGS.iter() {
                if self.handle_package_list(package_list, *start_task) {
                    continue 'outer;
                }
            }
            std::thread::sleep(Duration::from_millis(1000));
        }
    }
}
