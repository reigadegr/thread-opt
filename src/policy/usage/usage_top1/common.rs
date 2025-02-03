use crate::{
    cgroup::group_info::{get_background_group, get_middle_group, get_top_group},
    utils::affinity_setter::bind_thread_to_cpu,
};
use hashbrown::HashMap;
use libc::pid_t;
#[cfg(debug_assertions)]
use log::debug;

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
    top: &'a [&'a [u8]],
    only6: &'a [&'a [u8]],
    only7: &'a [&'a [u8]],
    middle: &'a [&'a [u8]],
    backend: &'a [&'a [u8]],
}

impl<'a> Policy<'a> {
    pub const fn new(
        top: &'a [&'a [u8]],
        only6: &'a [&'a [u8]],
        only7: &'a [&'a [u8]],
        middle: &'a [&'a [u8]],
        backend: &'a [&'a [u8]],
    ) -> Self {
        Self {
            top,
            only6,
            only7,
            middle,
            backend,
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
        if self.backend.iter().any(|&prefix| comm.starts_with(prefix)) {
            return CmdType::Background;
        }
        CmdType::Middle
    }

    // 执行策略
    pub fn execute_policy(&self, task_map: &HashMap<pid_t, Vec<u8>>, first: pid_t) {
        #[cfg(debug_assertions)]
        let start = std::time::Instant::now();
        execute_task(&CmdType::Only7, first);

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
        CmdType::Top => bind_thread_to_cpu(get_top_group(), tid),
        CmdType::Only6 => {
            let top_group = get_top_group();
            if top_group == [6, 7] {
                bind_thread_to_cpu(&[6], tid);
                return;
            }
            bind_thread_to_cpu(get_middle_group(), tid);
        }
        CmdType::Only7 => bind_thread_to_cpu(&[7], tid),
        CmdType::Middle => bind_thread_to_cpu(get_middle_group(), tid),
        CmdType::Background => bind_thread_to_cpu(get_background_group(), tid),
    }
}
