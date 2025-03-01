use super::Looper;
use crate::{
    config::{self, UsageTop1},
    policy::{
        pkg_cfg::StartArgs,
        usage_top1::{self, common::CmdType},
    },
};
use log::info;

impl Looper {
    fn bind_usage_top1<F>(
        &mut self,
        start_task: F,
        policy: &config::Policy,
        comm: &[u8],
        cmd_type: &CmdType,
    ) where
        F: Fn(&mut StartArgs, &config::Policy, &[u8], &CmdType),
    {
        start_task(
            &mut StartArgs {
                activity_utils: &mut self.activity_utils,
                pid: self.pid,
            },
            policy,
            comm,
            cmd_type,
        );
        self.game_exit();
    }

    pub fn policy_usage_top1(&mut self, i: &UsageTop1) -> bool {
        for package in &i.packages {
            if package == self.global_package {
                info!("Detected target App: {}", self.global_package);
                self.bind_usage_top1(
                    usage_top1::cfg_start::start_task,
                    &i.policy,
                    &i.max_comm,
                    &i.max_comm_target,
                );
                return true;
            }
        }
        false
    }
}
