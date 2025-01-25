use crate::{
    affinity_utils::bind_thread_to_cpu,
    cgroup::group_info::{get_background_group, get_middle_group, get_top_group},
};
use libc::pid_t;
const TOP: [&str; 0] = [];
const MIDDLE: [&str; 1] = ["RHIThread"];
const BACKEND: [&str; 0] = [];
const TOP_REGEX: [&str; 1] = ["Thread-"];
const MIDDLE_REGEX: [&str; 1] = ["RenderThread"];

enum CmdType {
    Top,
    Middle,
    Background,
}

fn get_cmd_type(thread_name: &str) -> CmdType {
    if TOP.contains(&thread_name) {
        return CmdType::Top;
    }

    if MIDDLE.contains(&thread_name) {
        return CmdType::Middle;
    }

    if BACKEND.contains(&thread_name) {
        return CmdType::Background;
    }

    // 使用 starts_with 方法匹配线程
    for prev_name in TOP_REGEX {
        if thread_name.starts_with(prev_name) {
            return CmdType::Top;
        }
    }
    for prev_name in MIDDLE_REGEX {
        if thread_name.starts_with(prev_name) {
            return CmdType::Middle;
        }
    }
    CmdType::Middle
}

fn execute_task(cmd_type: &CmdType, tid: pid_t) {
    match cmd_type {
        CmdType::Top => bind_thread_to_cpu(get_top_group(), tid),
        CmdType::Middle => bind_thread_to_cpu(get_middle_group(), tid),
        CmdType::Background => bind_thread_to_cpu(get_background_group(), tid),
    }
}

pub fn start_task(tid: pid_t, thread_name: &str) {
    let thread_type = get_cmd_type(thread_name);
    execute_task(&thread_type, tid);
}
