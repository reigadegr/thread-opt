use super::common::execute_policy;
use crate::policy::pkg_cfg::StartArgs;
use compact_str::CompactString;
use flume::{bounded, Receiver, Sender};
use hashbrown::HashMap;
use libc::pid_t;
#[cfg(debug_assertions)]
use log::debug;
use once_cell::sync::Lazy;
use std::time::Duration;

// 定义别名
type ChannelType = (Sender<Vec<pid_t>>, Receiver<Vec<pid_t>>);

// // 使用别名定义全局变量
pub static UNNAME_TIDS: Lazy<ChannelType> = Lazy::new(|| bounded(0));

pub fn start_task(args: &mut StartArgs) {
    args.controller.init_game(*args.pid);
    // 获取全局通道的发送端
    let tx = &UNNAME_TIDS.0;
    // 获取全局通道的接收端
    // let rx = &UNNAME_TIDS.1;
    loop {
        let pid = args.activity_utils.top_app_utils.get_pid();
        if pid != args.pid {
            args.controller.init_default();
            return;
        }

        let task_map = args.activity_utils.tid_utils.get_task_map(*pid);

        let unname_tids = get_thread_tids(task_map, "Thread-");
        #[cfg(debug_assertions)]
        debug!("发送即将开始");
        tx.send(unname_tids).unwrap();
        #[cfg(debug_assertions)]
        debug!("发送已经完毕");

        args.controller.update_max_usage_tid();
        let Some(tid1) = args.controller.first_max_tid() else {
            #[cfg(debug_assertions)]

            debug!("获取不到first max tid，直接循环");

            std::thread::sleep(Duration::from_millis(500));
            continue;
        };

        let Some(tid2) = args.controller.second_max_tid() else {
            #[cfg(debug_assertions)]

            debug!("获取不到second max tid，直接循环");

            std::thread::sleep(Duration::from_millis(500));
            continue;
        };
        #[cfg(debug_assertions)]
        debug!("负载第一高:{tid1}\n第二高:{tid2}");
        execute_policy(task_map, tid1, tid2);
        std::thread::sleep(Duration::from_millis(2000));
    }
}

fn get_thread_tids(task_map: &HashMap<pid_t, CompactString>, prefix: &str) -> Vec<pid_t> {
    #[cfg(debug_assertions)]
    debug!("原始的task_map:{task_map:?}");
    task_map
        .iter()
        .filter_map(|(&tid, name)| {
            if name.starts_with(prefix) {
                Some(tid)
            } else {
                None
            }
        })
        .collect()
}
