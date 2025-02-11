use crate::{
    cgroup::group_info::get_top_group,
    utils::global_cpu_utils::{
        bind_tid_to_background, bind_tid_to_middle, bind_tid_to_only6, bind_tid_to_only7,
        bind_tid_to_top,
    },
};
use hashbrown::HashMap;
use libc::pid_t;
#[cfg(debug_assertions)]
use log::debug;

// 动态生成 CmdType 枚举
pub enum CmdType {
    Top,
    Middle,
    Background,
    Only6,
    Only7,
}

// 动态生成 Policy 结构体
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

    // 根据线程名称获取线程类型
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

    // 执行策略
    pub fn execute_policy(
        &self,
        task_map: &HashMap<pid_t, Vec<u8>>,
        first: pid_t,
        cmd_type: &CmdType,
    ) {
        #[cfg(debug_assertions)]
        let start = std::time::Instant::now();

        execute_task(cmd_type, first);

        for (&tid, comm) in task_map.iter().filter(|(&tid, _)| tid != first) {
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
        CmdType::Top => bind_tid_to_top(tid),
        CmdType::Only6 => {
            let top_group = get_top_group();
            if top_group == [6, 7] {
                bind_tid_to_only6(tid);
                return;
            }
            bind_tid_to_middle(tid);
        }
        CmdType::Only7 => bind_tid_to_only7(tid),
        CmdType::Middle => bind_tid_to_middle(tid),
        CmdType::Background => bind_tid_to_background(tid),
    }
}
