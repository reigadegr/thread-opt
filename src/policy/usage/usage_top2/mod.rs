mod common;
pub mod policy_party;
pub mod policy_top2;

use crate::policy::{
    pkg_cfg::StartArgs,
    usage::{check_some, get_thread_tids, UNNAME_TIDS},
};
use common::execute_policy;
use flume::Sender;
use libc::pid_t;
use likely_stable::unlikely;
#[cfg(debug_assertions)]
use log::debug;
use std::time::Duration;

struct StartTask<'b, 'a: 'b> {
    args: &'b mut StartArgs<'a>,
    tx: &'b Sender<Vec<pid_t>>,
}

impl<'b, 'a: 'b> StartTask<'b, 'a> {
    fn new(start_args: &'b mut StartArgs<'a>) -> Self {
        Self {
            args: start_args,
            tx: &UNNAME_TIDS.0,
        }
    }

    fn bind_tids(&mut self, tid1: pid_t, tid2: pid_t) {
        let task_map = self
            .args
            .activity_utils
            .tid_utils
            .get_task_map(self.args.pid);
        execute_policy(task_map, tid1, tid2);
    }

    fn update_tids(&mut self, comm_prefix: &[u8]) {
        let task_map = self
            .args
            .activity_utils
            .tid_utils
            .get_task_map(self.args.pid);
        let unname_tids = get_thread_tids(task_map, comm_prefix);

        #[cfg(debug_assertions)]
        debug!("发送即将开始");
        self.tx.send(unname_tids).unwrap();
        #[cfg(debug_assertions)]
        debug!("发送已经完毕，喵等待一段时间计算");
        // std::thread::sleep(Duration::from_millis(100));
        self.args.controller.update_max_usage_tid();
    }

    fn start_task(&mut self, comm_prefix1: &[u8], comm_prefix2: Option<&[u8]>) {
        self.args.controller.init_game(true);
        loop {
            std::thread::sleep(Duration::from_millis(2000));
            let pid = self.args.activity_utils.top_app_utils.get_pid();
            if unlikely(pid != self.args.pid) {
                self.args.controller.init_default();
                return;
            }

            self.update_tids(comm_prefix1);
            check_some! { tid1, self.args.controller.first_max_tid()};

            let tid2 = if let Some(prefix2) = comm_prefix2 {
                self.update_tids(prefix2);
                check_some! { tid2, self.args.controller.first_max_tid() };
                tid2
            } else {
                check_some! { tid2, self.args.controller.second_max_tid()};
                tid2
            };

            #[cfg(debug_assertions)]
            debug!("负载第一高:{tid1}\n第二高:{tid2}");
            self.bind_tids(tid1, tid2);
        }
    }
}
