use crate::policy::affinity_policy::{dualo_policy, only7_policy, tid_list_t2_policy};
use hashbrown::HashMap;
#[cfg(debug_assertions)]
use log::debug;
#[cfg(debug_assertions)]
use minstant::Instant;

// 定义线程类型
#[derive(serde::Deserialize)]
pub enum CmdType {
    Dualo,
    Only7,
}

// 执行策略
pub fn execute_policy(task_map: &HashMap<i32, [u8; 16]>, first: i32, second: i32) {
    execute_task(&CmdType::Only7, first);
    execute_task(&CmdType::Dualo, second);

    let filtered_keys: Vec<i32> = task_map
        .keys()
        .filter(|&&tid| tid != first && tid != second)
        .copied()
        .collect();

    #[cfg(debug_assertions)]
    let start = Instant::now();
    tid_list_t2_policy(&filtered_keys);

    #[cfg(debug_assertions)]
    {
        let end = start.elapsed();

        debug!("一轮绑定核心完成时间: {:?} 数组长度{}", end, task_map.len());
    }
}

// 执行线程绑定任务
fn execute_task(cmd_type: &CmdType, tid: i32) {
    match cmd_type {
        CmdType::Dualo => dualo_policy(tid),
        CmdType::Only7 => only7_policy(tid),
    }
}
