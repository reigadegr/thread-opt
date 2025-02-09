mod common;
pub mod policy_cocos;
pub mod policy_ue;
pub mod policy_unity;

// 定义宏，但不导出
macro_rules! name_match_policy {
    ($name:ident, $top:expr, $only6:expr, $only7:expr, $middle:expr, $background:expr) => {
        use likely_stable::unlikely;
        #[cfg(debug_assertions)]
        use log::debug;
        pub fn $name(args: &mut $crate::policy::pkg_cfg::StartArgs) {
            loop {
                let pid = args.activity_utils.top_app_utils.get_pid();
                if unlikely(pid != args.pid) {
                    return;
                }
                #[cfg(debug_assertions)]
                let start = std::time::Instant::now();
                let task_map = args.activity_utils.tid_utils.get_task_map(pid);
                super::common::Policy::new($top, $only6, $only7, $middle, $background)
                    .execute_policy(task_map);
                #[cfg(debug_assertions)]
                {
                    let end = start.elapsed();
                    debug!(
                        "单线程:一轮绑定核心完成时间: {:?} 数组长度{}",
                        end,
                        task_map.len()
                    );
                }
                std::thread::sleep(std::time::Duration::from_millis(2000));
            }
        }
    };
}

// 重新导出宏，使其在子模块中可用
use name_match_policy;
