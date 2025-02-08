pub mod lolm;
pub mod unnamed;

macro_rules! top1_policy {
    ($Top: ident, $Only6: ident, $Only7: ident, $Middle: ident, $Backend: ident) => {
        pub fn start_task(args: &mut StartArgs) {
            args.controller.init_game(true);
            // 获取全局通道的发送端
            let tx = &UNNAME_TIDS.0;

            let mut finish = false;
            let mut usage_top1 = 0;

            loop {
                let pid = args.activity_utils.top_app_utils.get_pid();
                if unlikely(pid != args.pid) {
                    args.controller.init_default();
                    return;
                }

                let task_map = args.activity_utils.tid_utils.get_task_map(*pid);
                if likely(finish) {
                    Policy::new(&$Top, &$Only6, &$Only7, &$Middle, &$Backend)
                        .execute_policy(task_map, usage_top1);
                    std::thread::sleep(Duration::from_millis(1000));
                } else {
                    let unname_tids = get_thread_tids(task_map, b"Thread-");
                    #[cfg(debug_assertions)]
                    debug!("发送即将开始");
                    tx.send(unname_tids).unwrap();
                    #[cfg(debug_assertions)]
                    debug!("发送已经完毕");
                    std::thread::sleep(Duration::from_millis(100));
                    args.controller.update_max_usage_tid();
                    check_some! {tid1, args.controller.first_max_tid(), "无法获取最大负载tid"};
                    usage_top1 = tid1;
                    args.controller.init_default();
                    finish = true;
                    #[cfg(debug_assertions)]
                    debug!("计算后最终结果为:{usage_top1}\n");
                    continue;
                }

                std::thread::sleep(Duration::from_millis(1000));
            }
        }
    };
}

macro_rules! top1_common {
    ($initial_cmd:ident) => {
        use crate::{
            cgroup::group_info::get_top_group,
            utils::global_cpu_utils::{
                bind_tid_to_background, bind_tid_to_middle, bind_tid_to_only6, bind_tid_to_only7,
                bind_tid_to_top,
            },
        };
        use hashbrown::HashMap;
        use libc::pid_t;
        #[cfg(debug_assertions)]
        use log::debug;

        // 动态生成 CmdType 枚举
        pub enum CmdType {
            Top,
            Middle,
            Background,
            Only6,
            Only7,
        }

        // 动态生成 Policy 结构体
        pub struct Policy<'a> {
            pub top: &'a [&'a [u8]],
            pub only6: &'a [&'a [u8]],
            pub only7: &'a [&'a [u8]],
            pub middle: &'a [&'a [u8]],
            pub background: &'a [&'a [u8]],
        }

        impl<'a> Policy<'a> {
            pub const fn new(
                top: &'a [&'a [u8]],
                only6: &'a [&'a [u8]],
                only7: &'a [&'a [u8]],
                middle: &'a [&'a [u8]],
                background: &'a [&'a [u8]],
            ) -> Self {
                Self {
                    top,
                    only6,
                    only7,
                    middle,
                    background,
                }
            }

            // 根据线程名称获取线程类型
            fn get_cmd_type(&self, comm: &[u8]) -> CmdType {
                if self.top.iter().any(|&prefix| comm.starts_with(prefix)) {
                    return CmdType::Top;
                }
                if self.only6.iter().any(|&prefix| comm.starts_with(prefix)) {
                    return CmdType::Only6;
                }
                if self.only7.iter().any(|&prefix| comm.starts_with(prefix)) {
                    return CmdType::Only7;
                }
                if self.middle.iter().any(|&prefix| comm.starts_with(prefix)) {
                    return CmdType::Middle;
                }
                if self
                    .background
                    .iter()
                    .any(|&prefix| comm.starts_with(prefix))
                {
                    return CmdType::Background;
                }
                CmdType::Middle
            }

            // 执行策略
            pub fn execute_policy(&self, task_map: &HashMap<pid_t, Vec<u8>>, first: pid_t) {
                #[cfg(debug_assertions)]
                let start = std::time::Instant::now();

                execute_task(&CmdType::$initial_cmd, first);

                for (&tid, comm) in task_map.iter().filter(|(&tid, _)| tid != first) {
                    let cmd_type = self.get_cmd_type(comm);
                    execute_task(&cmd_type, tid);
                }

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

        // 执行线程绑定任务
        fn execute_task(cmd_type: &CmdType, tid: pid_t) {
            match cmd_type {
                CmdType::Top => bind_tid_to_top(tid),
                CmdType::Only6 => {
                    let top_group = get_top_group();
                    if top_group == [6, 7] {
                        bind_tid_to_only6(tid);
                        return;
                    }
                    bind_tid_to_middle(tid);
                }
                CmdType::Only7 => bind_tid_to_only7(tid),
                CmdType::Middle => bind_tid_to_middle(tid),
                CmdType::Background => bind_tid_to_background(tid),
            }
        }
    };
}

// // 调用宏并传入参数，生成绑定 Only6 的代码
// top1_common!(Only6);

// // 调用宏并传入参数，生成绑定 Only7 的代码
// top1_common!(Only7);

use top1_policy;

use top1_common;
