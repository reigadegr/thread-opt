use crate::{
    activity::{
        get_tid_info::{get_process_name, TidUtils},
        get_top_tid::TopAppUtils,
    },
    cgroup::group_info::get_background_group,
    cpu_common::Controller,
    policy::pkg_cfg::{StartArgs, PACKAGE_CONFIGS},
    utils::affinity_setter::bind_tid_list_to_cgroup,
};

use compact_str::CompactString;
use libc::pid_t;
use log::info;
use std::time::Duration;

pub struct Looper {
    pid: pid_t,
    global_package: CompactString,
    top_app_utils: TopAppUtils,
    tid_utils: TidUtils,
    controller: Controller,
}

impl Looper {
    pub fn new(controller: Controller) -> Self {
        Self {
            pid: 0,
            global_package: CompactString::new(""),
            top_app_utils: TopAppUtils::new(),
            tid_utils: TidUtils::new(),
            controller,
        }
    }

    fn game_exit(&mut self) {
        info!("Exiting game");
        let tid_list = self.tid_utils.get_tid_list(self.pid);
        bind_tid_list_to_cgroup(get_background_group(), tid_list);
        self.controller.init_default();
    }

    fn start_bind_common<F>(&mut self, start_task: F)
    where
        // 传入函数的签名
        F: Fn(&mut StartArgs),
    {
        // let task_map = self.tid_utils.get_task_map(self.pid);
        start_task(&mut StartArgs {
            controller: &mut self.controller,
            top_app_utils: &mut self.top_app_utils,
            tid_utils: &mut self.tid_utils,
            pid: &mut self.pid,
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
