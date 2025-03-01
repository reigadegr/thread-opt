use crate::{
    activity::{ActivityUtils, get_tid_info::get_process_name},
    config::PROFILE,
    utils::{affinity_utils::global_cpu_utils::bind_list_to_background, sleep::sleep_secs},
};
use compact_str::CompactString;
use libc::pid_t;
use log::info;

pub struct Looper {
    pub activity_utils: ActivityUtils,
    pub global_package: CompactString,
    pub pid: pid_t,
}

impl Looper {
    pub fn new(activity_utils: ActivityUtils) -> Self {
        Self {
            activity_utils,
            global_package: CompactString::new(""),
            pid: -1,
        }
    }

    pub fn game_exit(&mut self) {
        info!("Exiting game\n");
        let tid_list = self.activity_utils.tid_utils.get_tid_list(self.pid);
        bind_list_to_background(tid_list);
        self.pid = -1;
        self.activity_utils.tid_utils.tid_info.task_map.clear();
        self.activity_utils.tid_utils.tid_info.tid_list.clear();
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

            for i in &PROFILE.comm_match {
                if self.policy_name_match(i) {
                    continue 'outer;
                }
            }

            for i in &PROFILE.usage_top1 {
                if self.policy_usage_top1(i) {
                    continue 'outer;
                }
            }

            for i in &PROFILE.usage_top2 {
                if self.policy_usage_top2(i) {
                    continue 'outer;
                }
            }
        }
    }
}
