use compact_str::CompactString;
use flume::{bounded, Receiver, Sender};
use hashbrown::HashMap;
use libc::pid_t;
use log::debug;
use once_cell::sync::Lazy;

pub mod usage_top1;
pub mod usage_top2;

// 定义别名
type ChannelType = (Sender<Vec<pid_t>>, Receiver<Vec<pid_t>>);

// // 使用别名定义全局变量
pub static UNNAME_TIDS: Lazy<ChannelType> = Lazy::new(|| bounded(0));

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
