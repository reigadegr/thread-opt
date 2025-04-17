use super::super::affinity_policy::{
    background_policy, dualo_policy, middle_policy, mono_policy, only7_policy, top_policy,
};
extern crate alloc;
use crate::config::ByteArray;
use hashbrown::HashMap;
use libc::pid_t;
#[cfg(debug_assertions)]
use log::debug;
#[cfg(debug_assertions)]
use minstant::Instant;
use super::trie::{CmdType, Trie};

// 定义线程类型
enum CmdType {
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
        let mut trie = Trie::new();
        
        for prefix in self.top {
            trie.insert(prefix, CmdType::Top);
        }
        for prefix in self.dualo {
            trie.insert(prefix, CmdType::Dualo);
        }
        for prefix in self.only7 {
            trie.insert(prefix, CmdType::Only7);
        }
        for prefix in self.middle {
            trie.insert(prefix, CmdType::Middle);
        }
        for prefix in self.mono {
            trie.insert(prefix, CmdType::Mono);
        }
        for prefix in self.background {
            trie.insert(prefix, CmdType::Background);
        }
        
        trie.search(comm).unwrap_or(CmdType::Middle)
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
        CmdType::Dualo => dualo_policy(tid),
        CmdType::Only7 => only7_policy(tid),
        CmdType::Middle => middle_policy(tid),
        CmdType::Mono => mono_policy(tid),
        CmdType::Background => background_policy(tid),
    }
}
