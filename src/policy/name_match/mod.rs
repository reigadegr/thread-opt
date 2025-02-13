mod common;
pub mod policies;

use crate::policy::pkg_cfg::StartArgs;
use common::Policy;
use likely_stable::unlikely;
#[cfg(debug_assertions)]
use log::debug;

struct StartTask<'b, 'a: 'b> {
    policy: &'b Policy<'b>,
    args: &'b mut StartArgs<'a>,
}

impl<'b, 'a: 'b> StartTask<'b, 'a> {
    const fn new(start_args: &'b mut StartArgs<'a>, policy: &'b Policy) -> Self {
        Self {
            policy,
            args: start_args,
        }
    }

    fn start_task(&mut self) {
        loop {
            std::thread::sleep(std::time::Duration::from_millis(2000));
            let pid = self.args.activity_utils.top_app_utils.get_pid();
            if unlikely(pid != self.args.pid) {
                return;
            }
            #[cfg(debug_assertions)]
            let start = std::time::Instant::now();
            let task_map = self.args.activity_utils.tid_utils.get_task_map(pid);
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

// 定义宏，但不导出
macro_rules! name_match_init {
    () => {
        use super::super::common::Policy;
        use crate::policy::pkg_cfg::StartArgs;
        pub fn start_task(args: &mut StartArgs<'_>) {
            let policy = Policy {
                top: &TOP,
                only6: &ONLY6,
                only7: &ONLY7,
                middle: &MIDDLE,
                background: &BACKEND,
            };
            super::super::StartTask::new(args, &policy).start_task();
        }
    };
}

// 重新导出宏，使其在子模块中可用
use name_match_init;
