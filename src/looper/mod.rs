use super::activity::get_tid_info::TidUtils;
use super::activity::get_tid_info::get_process_name;
use super::activity::get_top_tid::TopAppUtils;
use crate::affinity_policy::{policy_normal, policy_pubg};
use crate::fs_utils::node_writer::write_node;
use crate::get_background_dir;
use log::info;
use std::time::Duration;

const NORMAL_PACKAGE: [&str; 6] = [
    "com.miHoYo.Yuanshen",
    "com.miHoYo.hkrpg",
    "com.tencent.tmgp.sgame",
    "com.miHoYo.Nap",
    "com.kurogame.mingchao",
    "com.yongshi.tenojo.ys",
];

const PUBG_PACKAGE: [&str; 1] = ["com.tencent.tmgp.pubgmhd"];
pub struct Looper {
    pid: i32,
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
        F: Fn(&i32, &str),
    {
        loop {
            let pid = self.top_app_utils.get_pid();
            if pid != &self.pid {
                info!("退出游戏");
                let tid_list = self.tid_utils.get_tid_list(&self.pid);
                for tid in tid_list {
                    write_node(get_background_dir(), tid);
                }
                return;
            }
            let task_map = self.tid_utils.get_task_map(pid);
            for (tid, comm) in task_map {
                start_task(tid, comm);
            }
            std::thread::sleep(Duration::from_millis(2000));
        }
    }

    fn handle_package_list<F>(&mut self, package_list: &[&str], start_task: F) -> bool
    where
        F: Fn(&i32, &str),
    {
        for &package in package_list {
            if package == self.global_package {
                info!("监听到目标App: {}", self.global_package);
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
                let name = get_process_name(pid).unwrap_or_default();

                if self.global_package == name {
                    std::thread::sleep(Duration::from_millis(1000));
                    continue 'outer;
                }
                self.global_package = name;
            }

            if self.handle_package_list(&NORMAL_PACKAGE, policy_normal::start_task) {
                continue 'outer;
            }
            if self.handle_package_list(&PUBG_PACKAGE, policy_pubg::start_task) {
                continue 'outer;
            }
            std::thread::sleep(Duration::from_millis(1000));
        }
    }
}
