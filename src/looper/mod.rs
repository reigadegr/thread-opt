use super::activity::get_tid_info::TidUtils;
use super::activity::get_tid_info::get_process_name;
use super::activity::get_top_tid::TopAppUtils;
use crate::affinity_set::policy::execute_task;
use crate::affinity_set::policy::get_cmd_type;
use log::info;
use std::time::Duration;

const PACKAGE: [&str; 5] = [
    "bin.mt.plus",
    "com.miHoYo.Yuanshen",
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

    pub fn enter_loop(&mut self) {
        let mut global_package = "".to_string();
        loop {
            let pid = self.top_app_utils.get_pid();
            let name = get_process_name(pid).unwrap_or_default();

            if global_package == name {
                info!("直接返回: 包名:-{}-", name);
                std::thread::sleep(Duration::from_millis(1000));
                continue;
            }
            global_package = name.clone();
            for i in PACKAGE {
                if i == name {
                    info!("包名:-{}-", name);

                    let task_map = self.tid_utils.get_task_map(pid);
                    // info!("{:?}", tids);
                    for (tid, comm) in task_map {
                        let thread_type = get_cmd_type(comm);
                        execute_task(thread_type, tid);
                    }

                    // let tl2 = self.tid_utils.get_tid_list(pid);
                    // info!("{:?}", tl2);
                    break;
                }
            }
            std::thread::sleep(Duration::from_millis(1000));
        }
    }
}
