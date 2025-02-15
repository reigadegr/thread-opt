mod common;
pub mod policy_party;
pub mod policy_top2;
use crate::{
    cpu_common::process_monitor::get_high_usage_tids,
    policy::{pkg_cfg::StartArgs, usage::get_thread_tids},
};
use common::execute_policy;
use libc::pid_t;
use likely_stable::unlikely;
#[cfg(debug_assertions)]
use log::debug;
use std::time::Duration;

struct StartTask<'b, 'a: 'b> {
    args: &'b mut StartArgs<'a>,
}

impl<'b, 'a: 'b> StartTask<'b, 'a> {
    const fn new(start_args: &'b mut StartArgs<'a>) -> Self {
        Self { args: start_args }
    }

    fn bind_tids(&mut self, tid1: pid_t, tid2: pid_t) {
        let task_map = self
            .args
            .activity_utils
            .tid_utils
            .get_task_map(self.args.pid);
        execute_policy(task_map, tid1, tid2);
    }

    fn update_tids(&mut self, comm_prefix: &[u8]) -> (pid_t, pid_t) {
        let task_map = self
            .args
            .activity_utils
            .tid_utils
            .get_task_map(self.args.pid);
        let unname_tids = get_thread_tids(task_map, comm_prefix);
        let (tid1, tid2) = get_high_usage_tids(&unname_tids);
        (tid1, tid2)
    }

    fn start_task(&mut self, comm_prefix1: &[u8], comm_prefix2: Option<&[u8]>) {
        loop {
            std::thread::sleep(Duration::from_millis(2000));

            let pid = self.args.activity_utils.top_app_utils.get_pid();
            if unlikely(pid != self.args.pid) {
                return;
            }
            #[cfg(debug_assertions)]
            let start = std::time::Instant::now();

            let (tid1, mut tid2) = self.update_tids(comm_prefix1);

            if let Some(prefix2) = comm_prefix2 {
                let (new_tid1, _) = self.update_tids(prefix2);
                tid2 = new_tid1;
            }

            #[cfg(debug_assertions)]
            debug!("负载第一高:{tid1}\n第二高:{tid2}");
            self.bind_tids(tid1, tid2);
            #[cfg(debug_assertions)]
            {
                let end = start.elapsed();
                debug!("top2一轮全部完成时间: {:?} ", end);
            }
        }
    }
}
