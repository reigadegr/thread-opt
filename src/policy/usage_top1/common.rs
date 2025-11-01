use super::super::affinity_policy::{
    background_policy, dualo_policy, middle_policy, mono_policy, only7_policy, top_policy,
};

use crate::config::ByteArray;
use hashbrown::HashMap;
#[cfg(debug_assertions)]
use log::debug;
#[cfg(debug_assertions)]
use minstant::Instant;

// 动态生成 CmdType 枚举
#[derive(serde::Deserialize, Eq, Hash, PartialEq)]
pub enum CmdType {
    Top,
    Middle,
    Mono,
    Background,
    Dualo,
    Only7,
}

pub struct Policy<'a> {
    pub top: &'a [ByteArray],
    pub dualo: &'a [ByteArray],
    pub only7: &'a [ByteArray],
    pub middle: &'a [ByteArray],
    pub mono: &'a [ByteArray],
    pub background: &'a [ByteArray],
}

impl Policy<'_> {
    pub const fn new(policy: &Self) -> Self {
        Self {
            top: policy.top,
            dualo: policy.dualo,
            only7: policy.only7,
            middle: policy.middle,
            mono: policy.mono,
            background: policy.background,
        }
    }

    fn get_cmd_type(&self, comm: &[u8]) -> CmdType {
        if self.top.iter().any(|prefix| comm.starts_with(prefix)) {
            return CmdType::Top;
        }
        if self.dualo.iter().any(|prefix| comm.starts_with(prefix)) {
            return CmdType::Dualo;
        }
        if self.only7.iter().any(|prefix| comm.starts_with(prefix)) {
            return CmdType::Only7;
        }
        if self.middle.iter().any(|prefix| comm.starts_with(prefix)) {
            return CmdType::Middle;
        }
        if self.mono.iter().any(|prefix| comm.starts_with(prefix)) {
            return CmdType::Mono;
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

    // 执行策略
    pub fn execute_policy(
        &self,
        task_map: &HashMap<i32, [u8; 16]>,
        first: i32,
        cmd_type: &CmdType,
    ) {
        #[cfg(debug_assertions)]
        let start = Instant::now();

        execute_task(cmd_type, first);

        for (&tid, comm) in task_map.iter().filter(|&(&tid, _)| tid != first) {
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
fn execute_task(cmd_type: &CmdType, tid: i32) {
    match cmd_type {
        CmdType::Top => top_policy(tid),
        CmdType::Dualo => dualo_policy(tid),
        CmdType::Only7 => only7_policy(tid),
        CmdType::Middle => middle_policy(tid),
        CmdType::Mono => mono_policy(tid),
        CmdType::Background => background_policy(tid),
    }
}
