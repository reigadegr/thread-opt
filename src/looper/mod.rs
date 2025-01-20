use super::activity::get_tid_info::TidUtils;
use super::activity::get_tid_info::get_process_name;
use super::activity::get_top_tid::TopAppUtils;
use crate::affinity_set::policy::execute_task;
use crate::affinity_set::policy::get_cmd_type;
use crate::fs_utils::node_writer::write_node;
use crate::get_background_dir;
use log::info;
use std::time::Duration;

const PACKAGE: [&str; 7] = [
    "com.miHoYo.Yuanshen",
    "com.miHoYo.hkrpg",
    "com.tencent.tmgp.sgame",
    "com.miHoYo.Nap",
    "com.kurogame.mingchao",
    "com.tencent.tmgp.pubgmhd",
    "com.yongshi.tenojo.ys",
];

pub struct Looper {
    pid: i32,
    top_app_utils: TopAppUtils,
    tid_utils: TidUtils,
}

impl Looper {
    pub fn new() -> Self {
        Self {
            pid: 0,
            top_app_utils: TopAppUtils::new(),
            tid_utils: TidUtils::new(),
        }
    }

    fn start_bind(&mut self) {
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
                let thread_type = get_cmd_type(comm);
                execute_task(thread_type, tid);
            }

            std::thread::sleep(Duration::from_millis(1000));
        }
    }

    pub fn enter_loop(&mut self) {
        let mut global_package = "".to_string();
        loop {
            let pid = self.top_app_utils.get_pid();
            let name = get_process_name(pid).unwrap_or_default();

            if global_package == name {
                std::thread::sleep(Duration::from_millis(1000));
                continue;
            }
            global_package = name;
            for i in PACKAGE {
                if i == global_package {
                    info!("监听到目标App: {}", global_package);
                    self.pid = *pid;
                    self.start_bind();
                    break;
                }
            }
            std::thread::sleep(Duration::from_millis(1000));
        }
    }
}
