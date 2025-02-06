use super::common::execute_policy;
use crate::policy::{
    pkg_cfg::StartArgs,
    usage::{get_thread_tids, UNNAME_TIDS},
};
use hashbrown::HashSet;
use libc::pid_t;
use likely_stable::{likely, unlikely};
#[cfg(debug_assertions)]
use log::debug;
use std::time::Duration;

pub fn start_task(args: &mut StartArgs) {
    args.controller.init_game(true);
    // 获取全局通道的发送端
    let tx = &UNNAME_TIDS.0;
    // 创建一个HashSet<pid_t>
    let mut high_usage_tids: Option<HashSet<pid_t>> = Some(HashSet::new());

    let mut finish = false;

    let mut usage_top1 = 0;
    let mut usage_top2 = 0;
    let mut insert_count: u8 = 0;

    loop {
        let pid = args.activity_utils.top_app_utils.get_pid();
        if pid != args.pid {
            args.controller.init_default();
            return;
        }

        let task_map = args.activity_utils.tid_utils.get_task_map(*pid);

        if finish {
            execute_policy(task_map, usage_top1, usage_top2);
            std::thread::sleep(Duration::from_millis(800));
        } else {
            let unname_tids = get_thread_tids(task_map, b"Thread-");
            #[cfg(debug_assertions)]
            debug!("发送即将开始");
            tx.send(unname_tids).unwrap();
            #[cfg(debug_assertions)]
            debug!("发送已经完毕，喵等待一段时间计算");
            std::thread::sleep(Duration::from_millis(100));
            args.controller.update_max_usage_tid();
            let Some(tid1) = args.controller.first_max_tid() else {
                #[cfg(debug_assertions)]
                debug!("获取不到first max tid，直接循环");
                std::thread::sleep(Duration::from_millis(100));
                continue;
            };

            let Some(tid2) = args.controller.second_max_tid() else {
                #[cfg(debug_assertions)]
                debug!("获取不到second max tid，直接循环");
                std::thread::sleep(Duration::from_millis(100));
                continue;
            };

            if likely(insert_count < 25) {
                if let Some(set) = high_usage_tids.as_mut() {
                    set.insert(tid1);
                    set.insert(tid2);
                    #[cfg(debug_assertions)]
                    debug!("负载第一高:{tid1}\n第二高:{tid2}");
                    if unlikely(set.len() > 2) {
                        #[cfg(debug_assertions)]
                        debug!("检测到集合长度大于2，重新vote");
                        set.clear();
                        insert_count = 10;
                        continue;
                    }

                    insert_count += 1;
                }

                execute_policy(task_map, tid1, tid2);
            } else {
                // 可以通过获取线程亲和性更准确的硬亲和
                usage_top1 = tid1;
                usage_top2 = tid2;

                args.controller.init_default();
                finish = true;
                high_usage_tids = None;
                #[cfg(debug_assertions)]
                debug!("计算后最终结果为:{usage_top1}\n第二高:{usage_top2}");
                continue;
            }
        }

        std::thread::sleep(Duration::from_millis(1200));
    }
}
