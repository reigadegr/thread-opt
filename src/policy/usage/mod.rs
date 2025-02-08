use flume::{bounded, Receiver, Sender};
use hashbrown::HashMap;
use libc::pid_t;
use once_cell::sync::Lazy;

pub mod usage_top1;
pub mod usage_top2;

// 定义别名
type ChannelType = (Sender<Vec<pid_t>>, Receiver<Vec<pid_t>>);

// // 使用别名定义全局变量
pub static UNNAME_TIDS: Lazy<ChannelType> = Lazy::new(|| bounded(0));

fn get_thread_tids(task_map: &HashMap<pid_t, Vec<u8>>, prefix: &[u8]) -> Vec<pid_t> {
    task_map
        .iter()
        .filter(|(_, name)| name.starts_with(prefix))
        .map(|(&tid, _)| tid)
        .collect()
}

macro_rules! check_some {
    ($var:ident, $expr:expr, $message:expr) => {
        let Some($var) = $expr else {
            #[cfg(debug_assertions)]
            debug!($message);
            std::thread::sleep(Duration::from_millis(100));
            continue;
        };
    };
}
use check_some;
