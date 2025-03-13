pub mod cfg_start;
mod common;

use crate::{
    policy::pkg_cfg::StartArgs,
    utils::{guard::DirGuard, node_reader::get_proc_path, sleep::sleep_secs},
};
use common::Policy;
use libc::{DIR, opendir};
use likely_stable::unlikely;
#[cfg(debug_assertions)]
use log::debug;
#[cfg(debug_assertions)]
use minstant::Instant;

struct StartTask<'b, 'a: 'b> {
    policy: &'b Policy<'b>,
    args: &'b mut StartArgs<'a>,
    dir_ptr: *mut DIR,
}

impl<'b, 'a: 'b> StartTask<'b, 'a> {
    fn new(start_args: &'b mut StartArgs<'a>, policy: &'b Policy) -> Self {
        let task_dir = get_proc_path::<32, 5>(start_args.pid, b"/task");
        let dir_ptr = unsafe { opendir(task_dir.as_ptr()) };

        Self {
            policy,
            args: start_args,
            dir_ptr,
        }
    }

    fn start_task(&mut self) {
        if unlikely(self.dir_ptr.is_null()) {
            return;
        }
        let _dir_ptr_guard = DirGuard::new(self.dir_ptr);
        loop {
            sleep_secs(1);
            let pid = self.args.activity_utils.top_app_utils.get_top_pid();
            if unlikely(pid != self.args.pid) {
                return;
            }
            #[cfg(debug_assertions)]
            let start = Instant::now();
            let task_map = self
                .args
                .activity_utils
                .tid_utils
                .get_task_map(pid, self.dir_ptr);
            common::Policy::new(self.policy).execute_policy(task_map);
            #[cfg(debug_assertions)]
            {
                let end = start.elapsed();
                debug!(
                    "单线程:一轮绑定核心完成时间: {:?} 数组长度{}",
                    end,
                    task_map.len()
                );
            }
        }
    }
}
