use super::activity::get_tid_info::TidUtils;
use super::activity::get_tid_info::get_process_name;
use super::activity::get_top_tid::TopAppUtils;
use crate::affinity_policy::{policy_normal, policy_pubg};
use crate::fs_utils::node_writer::write_node;
use crate::get_background_dir;
use log::info;
use std::time::Duration;

const NORMAL_PACKAGE: [&str; 7] = [
    "com.miHoYo.Yuanshen",
    "com.miHoYo.hkrpg",
    "com.tencent.tmgp.sgame",
    "com.miHoYo.Nap",
    "com.kurogame.mingchao",
    "com.tencent.tmgp.pubgmhd",
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

    fn start_bind_normal(&mut self) {
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
                let thread_type = policy_normal::get_cmd_type(comm);
                policy_normal::execute_task(thread_type, tid);
            }
            std::thread::sleep(Duration::from_millis(1000));
        }
    }
    fn start_bind_pubg(&mut self) {
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
                let thread_type = policy_pubg::get_cmd_type(comm);
                policy_pubg::execute_task(thread_type, tid);
            }
            std::thread::sleep(Duration::from_millis(1000));
        }
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

            let pid = self.top_app_utils.get_pid();
            for i in NORMAL_PACKAGE {
                if i == self.global_package {
                    info!("监听到目标App: {}", self.global_package);
                    self.pid = *pid;
                    self.start_bind_normal();
                    continue 'outer;
                }
            }

            for i in PUBG_PACKAGE {
                if i == self.global_package {
                    info!("监听到目标App: {}", self.global_package);
                    self.pid = *pid;
                    self.start_bind_pubg();
                    continue 'outer;
                }
            }

            std::thread::sleep(Duration::from_millis(1000));
        }
    }
}
