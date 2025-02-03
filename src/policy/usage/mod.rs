use compact_str::CompactString;
use flume::{bounded, Receiver, Sender};
use hashbrown::{HashMap, HashSet};
use libc::pid_t;
use once_cell::sync::Lazy;

pub mod usage_top1;
pub mod usage_top2;

// 定义别名
type ChannelType = (Sender<HashSet<pid_t>>, Receiver<HashSet<pid_t>>);

// // 使用别名定义全局变量
pub static UNNAME_TIDS: Lazy<ChannelType> = Lazy::new(|| bounded(0));

fn get_thread_tids(task_map: &HashMap<pid_t, CompactString>, prefix: &str) -> HashSet<pid_t> {
    task_map
        .iter()
        .filter(|(_, name)| name.starts_with(prefix))
        .map(|(&tid, _)| tid)
        .collect()
}
