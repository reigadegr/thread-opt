use crate::{
    cgroup::group_info::{get_background_group, get_middle_group, get_top_group},
    utils::affinity_setter::bind_thread_to_cpu,
};
use compact_str::CompactString;
use hashbrown::HashMap;
use libc::pid_t;

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
    top: &'a [&'a str],
    only6: &'a [&'a str],
    only7: &'a [&'a str],
    middle: &'a [&'a str],
    backend: &'a [&'a str],
}

impl<'a> Policy<'a> {
    // 构造函数
    pub const fn new(
        top: &'a [&'a str],
        only6: &'a [&'a str],
        only7: &'a [&'a str],
        middle: &'a [&'a str],
        backend: &'a [&'a str],
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
    fn get_cmd_type(&self, comm: &str) -> CmdType {
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
    pub fn execute_policy(&self, task_map: &HashMap<pid_t, CompactString>) {
        for (tid, comm) in task_map {
            let cmd_type = self.get_cmd_type(comm);
            execute_task(&cmd_type, *tid);
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
