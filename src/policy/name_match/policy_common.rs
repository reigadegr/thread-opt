// src/policy/policy_common.rs
use crate::{
    utils::affinity_setter::bind_thread_to_cpu,
    cgroup::group_info::{get_background_group, get_middle_group, get_top_group},
};
use libc::pid_t;

// 定义通用的 CmdType 枚举
pub enum CmdType {
    Top,
    Middle,
    Background,
    Only6,
    Only7,
}

// 定义通用的 get_cmd_type 函数
pub fn get_cmd_type(
    thread: &str,
    top: &[&str],
    only6: &[&str],
    only7: &[&str],
    middle: &[&str],
    backend: &[&str],
) -> CmdType {
    if top.iter().any(|&prefix| thread.starts_with(prefix)) {
        return CmdType::Top;
    }
    if only6.iter().any(|&prefix| thread.starts_with(prefix)) {
        return CmdType::Only6;
    }
    if only7.iter().any(|&prefix| thread.starts_with(prefix)) {
        return CmdType::Only7;
    }
    if middle.iter().any(|&prefix| thread.starts_with(prefix)) {
        return CmdType::Middle;
    }
    if backend.iter().any(|&prefix| thread.starts_with(prefix)) {
        return CmdType::Background;
    }
    CmdType::Middle
}

// 定义通用的 execute_task 函数
pub fn execute_task(cmd_type: &CmdType, tid: pid_t) {
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
