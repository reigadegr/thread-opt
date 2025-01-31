use crate::{
    cgroup::group_info::{get_middle_group, get_top_group},
    utils::affinity_setter::{bind_thread_to_cpu, bind_tid_list_to_cgroup},
};
use compact_str::CompactString;
use hashbrown::HashMap;
use libc::pid_t;
#[cfg(debug_assertions)]
use log::debug;

// 定义线程类型
enum CmdType {
    Only6,
    Only7,
}

// 执行策略
pub fn execute_policy(task_map: &HashMap<pid_t, CompactString>, first: pid_t, second: pid_t) {
    #[cfg(debug_assertions)]
    let start = std::time::Instant::now();
    execute_task(&CmdType::Only7, first);
    execute_task(&CmdType::Only6, second);

    let filtered_keys: Vec<pid_t> = task_map
        .keys()
        .filter(|&&tid| tid != first && tid != second)
        .copied()
        .collect();
    bind_tid_list_to_cgroup(get_middle_group(), &filtered_keys);
    // for (&tid, comm) in task_map
    // .iter()
    // .filter(|(&tid, _)| tid != first && tid != second)
    // {
    // let cmd_type = self.get_cmd_type(comm);
    // execute_task(&cmd_type, tid);
    // }

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

// 执行线程绑定任务
fn execute_task(cmd_type: &CmdType, tid: pid_t) {
    match cmd_type {
        CmdType::Only6 => {
            let top_group = get_top_group();
            if top_group == [6, 7] {
                bind_thread_to_cpu(&[6], tid);
                return;
            }
            bind_thread_to_cpu(get_middle_group(), tid);
        }
        CmdType::Only7 => bind_thread_to_cpu(&[7], tid),
    }
}
