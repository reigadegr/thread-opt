pub mod cfg_start;
pub mod common;

use super::get_thread_tids;
use crate::{
    cpu_common::process_monitor::get_top1_tid, policy::pkg_cfg::StartArgs, utils::sleep::sleep_secs,
};
use common::{CmdType, Policy};
use libc::pid_t;
use likely_stable::unlikely;
#[cfg(debug_assertions)]
use log::debug;

struct StartTask<'b, 'a: 'b> {
    policy: &'b Policy<'b>,
    args: &'b mut StartArgs<'a>,
    usage_top1: pid_t,
}

impl<'b, 'a: 'b> StartTask<'b, 'a> {
    const fn new(start_args: &'b mut StartArgs<'a>, policy: &'b Policy) -> Self {
        Self {
            policy,
            args: start_args,
            usage_top1: 0,
        }
    }

    fn after_usage_task(&mut self, cmd_type: &CmdType) {
        let task_map = self
            .args
            .activity_utils
            .tid_utils
            .get_task_map(self.args.pid);
        Policy::new(self.policy).execute_policy(task_map, self.usage_top1, cmd_type);
    }

    fn get_new_tid(&mut self, comm_prefix: &[u8]) -> pid_t {
        let task_map = self
            .args
            .activity_utils
            .tid_utils
            .get_task_map(self.args.pid);
        let unname_tids = get_thread_tids(task_map, comm_prefix);
        get_top1_tid(&unname_tids)
    }

    fn set_new_tid(&mut self, comm_prefix: &[u8]) {
        let tid1 = self.get_new_tid(comm_prefix);
        self.usage_top1 = tid1;
        #[cfg(debug_assertions)]
        debug!("计算后最终结果为:{0}\n", self.usage_top1);
    }

    fn start_task(&mut self, comm_prefix: &[u8], cmd_type: &CmdType) {
        loop {
            sleep_secs(2);
            let pid = self.args.activity_utils.top_app_utils.get_top_pid();
            if unlikely(pid != self.args.pid) {
                return;
            }
            self.set_new_tid(comm_prefix);
            self.after_usage_task(cmd_type);
        }
    }
}
