use crate::{
    activity::{ActivityUtils, get_tid_info::get_process_name, get_tid_info::read_task_dir},
    config::PROFILE,
    utils::affinity_utils::global_cpu_utils::bind_list_to_background,
};
use anyhow::Result;
use compact_str::CompactString;
use log::info;

pub struct Looper {
    pub activity_utils: ActivityUtils,
    pub global_package: CompactString,
    pub pid: i32,
}

impl Looper {
    pub fn new(activity_utils: ActivityUtils) -> Self {
        Self {
            activity_utils,
            global_package: CompactString::new(""),
            pid: -1,
        }
    }

    pub fn game_exit(&mut self) -> Result<()> {
        info!("Exiting game\n");
        let tid_list = read_task_dir(self.pid)?;
        bind_list_to_background(&tid_list);
        self.pid = -1;
        self.activity_utils.tid_utils.tid_info.task_map.clear();
        Ok(())
    }

    pub async fn enter_loop(&mut self) {
        'outer: loop {
            {
                let pid = self.activity_utils.top_app_utils.get_top_pid();
                if self.pid == pid {
                    continue 'outer;
                }
                self.pid = pid;
                let name = get_process_name(pid).await.unwrap_or_default();
                self.global_package = name;
            }

            for i in &PROFILE.comm_match {
                if self.policy_name_match(i).await {
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
