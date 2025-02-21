use super::super::affinity_policy::{only6_policy, only7_policy};
use crate::{
    cgroup::group_info::{get_background_group, get_middle_group, get_top_group},
    utils::affinity_utils::global_cpu_utils::{
        bind_list_to_middle, bind_list_to_middle_background, bind_list_to_zero_five,
    },
};

extern crate alloc;
use alloc::vec::Vec;

use hashbrown::HashMap;
use libc::pid_t;
#[cfg(debug_assertions)]
use log::debug;
#[cfg(debug_assertions)]
use minstant::Instant;

// 定义线程类型
enum CmdType {
    Only6,
    Only7,
}

// 执行策略
pub fn execute_policy(
    task_map: &HashMap<pid_t, heapless::Vec<u8, 16>>,
    first: pid_t,
    second: pid_t,
) {
    execute_task(&CmdType::Only7, first);
    execute_task(&CmdType::Only6, second);

    let filtered_keys: Vec<pid_t> = task_map
        .keys()
        .filter(|&&tid| tid != first && tid != second)
        .copied()
        .collect();

    let background_group = get_background_group();
    let middle_group = get_middle_group();
    let top_group = get_top_group();

    #[cfg(debug_assertions)]
    let start = Instant::now();
    if background_group == middle_group {
        if top_group.len() == 4 {
            bind_list_to_zero_five(&filtered_keys);
        } else {
            bind_list_to_middle(&filtered_keys);
        }
    } else {
        bind_list_to_middle_background(&filtered_keys);
    }

    #[cfg(debug_assertions)]
    {
        let end = start.elapsed();

        debug!("一轮绑定核心完成时间: {:?} 数组长度{}", end, task_map.len());
    }
}

// 执行线程绑定任务
fn execute_task(cmd_type: &CmdType, tid: pid_t) {
    match cmd_type {
        CmdType::Only6 => only6_policy(tid),
        CmdType::Only7 => only7_policy(tid),
    }
}
