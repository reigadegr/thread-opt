pub mod cfg_start;
pub mod common;
use super::get_thread_tids;
use crate::{
    cpu_common::process_monitor::{get_top1_tid, get_top2_tids},
    policy::pkg_cfg::StartArgs,
    utils::{guard::DirGuard, node_reader::get_proc_path},
};

use common::execute_policy;
use libc::{DIR, opendir};
use likely_stable::unlikely;
#[cfg(debug_assertions)]
use log::debug;

struct StartTask<'b, 'a: 'b> {
    args: &'b mut StartArgs<'a>,
}

impl<'b, 'a: 'b> StartTask<'b, 'a> {
    fn new(start_args: &'b mut StartArgs<'a>) -> Self {
        let task_dir = get_proc_path::<32>(start_args.pid, b"/task");
        let dir_ptr = unsafe { opendir(task_dir.as_ptr()) };
        Self { args: start_args }
    }

    async fn bind_tids(&mut self, tid1: i32, tid2: i32) {
        let task_map = self
            .args
            .activity_utils
            .tid_utils
            .get_task_map(self.args.pid)
            .await;
        execute_policy(task_map, tid1, tid2);
    }

    async fn audit_tids(&mut self, comm_prefix1: &[u8], comm_prefix2: Option<&[u8]>) {
        let task_map = self
            .args
            .activity_utils
            .tid_utils
            .get_task_map(self.args.pid)
            .await;

        let prefix_tids = get_thread_tids(task_map, comm_prefix1);

        if let Some(prefix2) = comm_prefix2 {
            let tid1 = get_top1_tid(&prefix_tids).await;

            let prefix_tids = get_thread_tids(task_map, prefix2);
            let tid2 = get_top1_tid(&prefix_tids).await;

            #[cfg(debug_assertions)]
            debug!("负载第一高:{tid1}\n第二高:{tid2}");
            self.bind_tids(tid1, tid2);
            return;
        }

        let (tid1, tid2) = get_top2_tids(&prefix_tids).await;
        #[cfg(debug_assertions)]
        debug!("负载第一高:{tid1}\n第二高:{tid2}");
        self.bind_tids(tid1, tid2);
    }

    async fn start_task(&mut self, comm_prefix1: &[u8], comm_prefix2: Option<&[u8]>) {
        loop {
            let pid = self.args.activity_utils.top_app_utils.get_top_pid();
            if unlikely(pid != self.args.pid) {
                return;
            }

            self.audit_tids(comm_prefix1, comm_prefix2).await;
        }
    }
}
