pub mod macro_common;
pub mod policies;
use crate::cpu_common::process_monitor::get_high_usage_tids;
use crate::policy::{pkg_cfg::StartArgs, usage::get_thread_tids};
use libc::pid_t;
use likely_stable::{likely, unlikely};
#[cfg(debug_assertions)]
use log::debug;
use macro_common::{CmdType, Policy};
use std::time::Duration;

struct StartTask<'b, 'a: 'b> {
    policy: &'b Policy<'b>,
    args: &'b mut StartArgs<'a>,
    usage_top1: pid_t,
    finish: bool,
}

impl<'b, 'a: 'b> StartTask<'b, 'a> {
    const fn new(start_args: &'b mut StartArgs<'a>, policy: &'b Policy) -> Self {
        Self {
            policy,
            args: start_args,
            usage_top1: 0,
            finish: false,
        }
    }

    fn after_usage_task(&mut self, cmd_type: &CmdType) {
        let task_map = self
            .args
            .activity_utils
            .tid_utils
            .get_task_map(self.args.pid);
        Policy::new(self.policy).execute_policy(task_map, self.usage_top1, cmd_type);
        std::thread::sleep(Duration::from_millis(1000));
    }

    fn change_to_finish_state(&mut self, tid1: pid_t) {
        self.usage_top1 = tid1;
        self.finish = true;
        #[cfg(debug_assertions)]
        debug!("计算后最终结果为:{0}\n", self.usage_top1);
    }

    fn update_tids(&mut self, comm_prefix: &[u8]) -> pid_t {
        let task_map = self
            .args
            .activity_utils
            .tid_utils
            .get_task_map(self.args.pid);
        let unname_tids = get_thread_tids(task_map, comm_prefix);
        let (tid1, _) = get_high_usage_tids(&unname_tids);
        tid1
    }

    fn start_task(&mut self, comm_prefix: &[u8], cmd_type: &CmdType) {
        loop {
            std::thread::sleep(Duration::from_millis(1000));
            let pid = self.args.activity_utils.top_app_utils.get_pid();
            if unlikely(pid != self.args.pid) {
                return;
            }

            if likely(self.finish) {
                self.after_usage_task(cmd_type);
            } else {
                let tid1 = self.update_tids(comm_prefix);
                self.change_to_finish_state(tid1);
            }
        }
    }
}

macro_rules! top1_macro_init {
    ($CommPrefix:expr,$initial_cmd:ident) => {
        use super::super::macro_common::{CmdType, Policy};
        use crate::policy::pkg_cfg::StartArgs;
        pub fn start_task(args: &mut StartArgs<'_>) {
            let policy = Policy {
                top: &TOP,
                only6: &ONLY6,
                only7: &ONLY7,
                middle: &MIDDLE,
                background: &BACKEND,
            };
            super::super::StartTask::new(args, &policy)
                .start_task($CommPrefix, &CmdType::$initial_cmd);
        }
    };
}

use top1_macro_init;
