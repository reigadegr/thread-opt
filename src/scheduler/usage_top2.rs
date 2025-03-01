use super::Looper;
use crate::{
    config::{UsageTop2},
    policy::{pkg_cfg::StartArgs, usage_top2::cfg_start},
};
use log::info;

impl Looper {
    fn bind_usage_top2<F>(&mut self, start_task: F, comm1: &[u8], comm2: Option<&[u8]>)
    where
        F: Fn(&mut StartArgs, &[u8], Option<&[u8]>),
    {
        start_task(
            &mut StartArgs {
                activity_utils: &mut self.activity_utils,
                pid: self.pid,
            },
            comm1,
            comm2,
        );
        self.game_exit();
    }

    pub fn policy_usage_top2(&mut self, i: &UsageTop2) -> bool {
        for package in &i.packages {
            if package == self.global_package {
                info!("Detected target App: {}", self.global_package);
                self.bind_usage_top2(cfg_start::start_task, &i.max_comm, i.second_comm.as_deref());
                return true;
            }
        }
        false
    }
}
