use super::super::affinity_policy::{
    background_policy, middle_policy, only6_policy, only7_policy, top_policy,
};
extern crate alloc;
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

type ByteArray = heapless::Vec<u8, 16>;

pub struct Policy<'a> {
    pub top: &'a [ByteArray],
    pub only6: &'a [ByteArray],
    pub only7: &'a [ByteArray],
    pub middle: &'a [ByteArray],
    pub background: &'a [ByteArray],
}

impl Policy<'_> {
    pub const fn new(policy: &Self) -> Self {
        Self {
            top: policy.top,
            only6: policy.only6,
            only7: policy.only7,
            middle: policy.middle,
            background: policy.background,
        }
    }

    fn get_cmd_type(&self, comm: &[u8]) -> CmdType {
        if self.top.iter().any(|prefix| comm.starts_with(prefix)) {
            return CmdType::Top;
        }
        if self.only6.iter().any(|prefix| comm.starts_with(prefix)) {
            return CmdType::Only6;
        }
        if self.only7.iter().any(|prefix| comm.starts_with(prefix)) {
            return CmdType::Only7;
        }
        if self.middle.iter().any(|prefix| comm.starts_with(prefix)) {
            return CmdType::Middle;
        }
        if self
            .background
            .iter()
            .any(|prefix| comm.starts_with(prefix))
        {
            return CmdType::Background;
        }
        CmdType::Middle
    }

    pub fn execute_policy(&self, task_map: &HashMap<pid_t, [u8; 16]>) {
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
        CmdType::Top => top_policy(tid),
        CmdType::Only6 => only6_policy(tid),
        CmdType::Only7 => only7_policy(tid),
        CmdType::Middle => middle_policy(tid),
        CmdType::Background => background_policy(tid),
    }
}
