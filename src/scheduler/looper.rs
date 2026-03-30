use crate::{
    activity::{
        ActivityUtils,
        get_tid_info::{get_process_name, read_task_dir},
    },
    config::AtomicConfig,
    utils::affinity_utils::global_cpu_utils::bind_list_to_background,
};
use anyhow::Result;
use compact_str::CompactString;
use log::info;
use std::sync::Arc;

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
        self.activity_utils.tid_utils.file_cache.clear();
        Ok(())
    }

    // 修复 E0596: 这里必须是 &mut self，因为内部修改了 self.pid 和 self.global_package
    pub fn enter_loop(&mut self, config_manager: &Arc<AtomicConfig>) {
        'outer: loop {
            {
                let pid = self.activity_utils.top_app_utils.get_top_pid();
                if self.pid == pid {
                    continue 'outer;
                }
                self.pid = pid;
                let name = get_process_name(pid).unwrap_or_default();
                self.global_package = name;
            }

            // 获取配置
            let config = config_manager.get();

            for i in &config.comm_match {
                if self.policy_name_match(i) {
                    continue 'outer;
                }
            }

            for i in &config.usage_top1 {
                if self.policy_usage_top1(i) {
                    continue 'outer;
                }
            }

            for i in &config.usage_top2 {
                if self.policy_usage_top2(i) {
                    continue 'outer;
                }
            }
        }
    }
}
