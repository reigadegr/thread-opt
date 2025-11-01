pub mod affinity_policy;
pub mod name_match;
pub mod pkg_cfg;
pub mod usage_top1;
pub mod usage_top2;

extern crate alloc;
use alloc::vec::Vec;
use hashbrown::HashMap;

fn get_thread_tids(task_map: &HashMap<i32, [u8; 16]>, prefix: &[u8]) -> Vec<i32> {
    task_map
        .iter()
        .filter(|(_, name)| name.starts_with(prefix))
        .map(|(&tid, _)| tid)
        .collect()
}
