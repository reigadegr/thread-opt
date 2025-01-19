use super::activity::get_tid_info::TidUtils;
use super::activity::get_tid_info::get_process_name;
use super::activity::get_top_tid::TopAppUtils;
use crate::affinity_set::policy::execute_task;
use crate::affinity_set::policy::get_cmd_type;
use log::info;
use std::time::Duration;

const PACKAGE: [&str; 5] = [
    "com.miHoYo.Yuanshen",
    "com.miHoYo.hkrpg",
    "com.tencent.tmgp.sgame",
    "com.miHoYo.Nap",
    "com.kurogame.mingchao",
];

pub struct Looper {
    top_app_utils: TopAppUtils,
    tid_utils: TidUtils,
}

impl Looper {
    pub fn new() -> Self {
        Self {
            top_app_utils: TopAppUtils::new(),
            tid_utils: TidUtils::new(),
        }
    }

    fn start_bind(&mut self, origin_name: &str) {
        loop {
            let pid = self.top_app_utils.get_pid();
            let name = get_process_name(pid).unwrap_or_default();
            if name != origin_name {
                return;
            }
            let task_map = self.tid_utils.get_task_map(pid);
            // info!("{:?}", tids);
            for (tid, comm) in task_map {
                let thread_type = get_cmd_type(comm);
                execute_task(thread_type, tid);
            }

            // let tl2 = self.tid_utils.get_tid_list(pid);
            // info!("{:?}", tl2);
            std::thread::sleep(Duration::from_millis(1000));
        }
    }

    pub fn enter_loop(&mut self) {
        let mut global_package = "".to_string();
        loop {
            let pid = self.top_app_utils.get_pid();
            let name = get_process_name(pid).unwrap_or_default();

            if global_package == name {
                // info!("直接返回: 包名:-{}-", name);
                std::thread::sleep(Duration::from_millis(1000));
                continue;
            }
            global_package = name.clone();
            for i in PACKAGE {
                if i == name {
                    info!("监听到目标App: {}", name);
                    self.start_bind(&global_package);
                    break;
                }
            }
            std::thread::sleep(Duration::from_millis(1000));
        }
    }
}
