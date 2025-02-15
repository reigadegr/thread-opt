use hashbrown::HashMap;
use libc::pid_t;

pub mod usage_top1;
pub mod usage_top2;

fn get_thread_tids(task_map: &HashMap<pid_t, Vec<u8>>, prefix: &[u8]) -> Vec<pid_t> {
    task_map
        .iter()
        .filter(|(_, name)| name.starts_with(prefix))
        .map(|(&tid, _)| tid)
        .collect()
}
