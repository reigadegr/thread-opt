use crate::{
    cgroup::group_info::get_top_group,
    utils::{
        affinity_utils::global_cpu_utils::{
            bind_tid_to_background, bind_tid_to_middle, bind_tid_to_only6, bind_tid_to_only7,
            bind_tid_to_top,
        },
        priority_utils::global_priority_utils::{
            set_tid_to_background_priority, set_tid_to_middle_priority, set_tid_to_only6_priority,
            set_tid_to_only7_priority, set_tid_to_top_priority,
        },
    },
};
use hashbrown::HashMap;
use libc::pid_t;
#[cfg(debug_assertions)]
use log::debug;
#[cfg(debug_assertions)]
use minstant::Instant;

// 定义线程类型
enum CmdType {
    Top,
    Middle,
    Background,
    Only6,
    Only7,
}

// 定义通用策略类
pub struct Policy<'a> {
    pub top: &'a [&'a [u8]],
    pub only6: &'a [&'a [u8]],
    pub only7: &'a [&'a [u8]],
    pub middle: &'a [&'a [u8]],
    pub background: &'a [&'a [u8]],
}

impl<'a> Policy<'a> {
    pub const fn new(policy: &'a Policy) -> Self {
        Self {
            top: policy.top,
            only6: policy.only6,
            only7: policy.only7,
            middle: policy.middle,
            background: policy.background,
        }
    }

    fn get_cmd_type(&self, comm: &[u8]) -> CmdType {
        if self.top.iter().any(|&prefix| comm.starts_with(prefix)) {
            return CmdType::Top;
        }
        if self.only6.iter().any(|&prefix| comm.starts_with(prefix)) {
            return CmdType::Only6;
        }
        if self.only7.iter().any(|&prefix| comm.starts_with(prefix)) {
            return CmdType::Only7;
        }
        if self.middle.iter().any(|&prefix| comm.starts_with(prefix)) {
            return CmdType::Middle;
        }
        if self
            .background
            .iter()
            .any(|&prefix| comm.starts_with(prefix))
        {
            return CmdType::Background;
        }
        CmdType::Middle
    }

    pub fn execute_policy(&self, task_map: &HashMap<pid_t, heapless::Vec<u8, 16>>) {
        #[cfg(debug_assertions)]
        let start = Instant::now();
        for (&tid, comm) in task_map {
            let cmd_type = self.get_cmd_type(comm);
            execute_task(&cmd_type, tid);
        }
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
}

// 执行线程绑定任务
fn execute_task(cmd_type: &CmdType, tid: pid_t) {
    match cmd_type {
        CmdType::Top => {
            set_tid_to_top_priority(tid);
            bind_tid_to_top(tid);
        }
        CmdType::Only6 => {
            let top_group = get_top_group();
            if top_group == [6, 7] {
                set_tid_to_only6_priority(tid);
                bind_tid_to_only6(tid);
                return;
            }
            set_tid_to_middle_priority(tid);
            bind_tid_to_middle(tid);
        }
        CmdType::Only7 => {
            set_tid_to_only7_priority(tid);
            bind_tid_to_only7(tid);
        }
        CmdType::Middle => {
            set_tid_to_middle_priority(tid);
            bind_tid_to_middle(tid);
        }
        CmdType::Background => {
            set_tid_to_background_priority(tid);
            bind_tid_to_background(tid);
        }
    }
}
