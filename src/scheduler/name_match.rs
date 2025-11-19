use super::Looper;
use crate::{
    config::{self, NameMatch},
    policy::{name_match::cfg_start, pkg_cfg::StartArgs},
};
use log::info;

impl Looper {
    fn bind_name_match<F>(&mut self, start_task: F, policy: &config::Policy)
    where
        F: Fn(&mut StartArgs, &config::Policy),
    {
        start_task(
            &mut StartArgs {
                activity_utils: &mut self.activity_utils,
                pid: self.pid,
            },
            policy,
        );
        let _ = self.game_exit();
    }

    pub async fn policy_name_match(&mut self, i: &NameMatch) -> bool {
        for package in &i.packages {
            if package == self.global_package {
                info!("Detected target App: {}", self.global_package);
                self.bind_name_match(cfg_start::start_task, &i.policy);
                return true;
            }
        }
        false
    }
}
