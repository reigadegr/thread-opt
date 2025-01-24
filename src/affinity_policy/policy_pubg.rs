use crate::affinity_utils::{
    analysis::{get_background_group, get_middle_group, get_top_group},
    bind_thread_to_cpu,
};
use libc::pid_t;
const TOP_THREADS: [&str; 0] = [];
const MIDDLE_THREADS: [&str; 1] = ["RHIThread"];
const BACKEND_THREADS: [&str; 0] = [];
const ALL_THREADS: [&str; 0] = [];
const TOP_REGEX_THREADS: [&str; 1] = ["Thread-"];
const MIDDLE_REGEX_THREADS: [&str; 1] = ["RenderThread"];

enum CmdType {
    All,
    Top,
    Middle,
    Background,
}

fn get_cmd_type(thread_name: &str) -> CmdType {
    if TOP_THREADS.contains(&thread_name) {
        return CmdType::Top;
    }

    if MIDDLE_THREADS.contains(&thread_name) {
        return CmdType::Middle;
    }

    if BACKEND_THREADS.contains(&thread_name) {
        return CmdType::Background;
    }

    if ALL_THREADS.contains(&thread_name) {
        return CmdType::All;
    }

    // 使用 starts_with 方法匹配线程
    for prev_name in TOP_REGEX_THREADS {
        if thread_name.starts_with(prev_name) {
            return CmdType::Top;
        }
    }
    for prev_name in MIDDLE_REGEX_THREADS {
        if thread_name.starts_with(prev_name) {
            return CmdType::Middle;
        }
    }
    CmdType::Middle
}

fn execute_task(cmd_type: CmdType, tid: &pid_t) {
    match cmd_type {
        CmdType::Top => bind_thread_to_cpu(get_top_group(), tid),
        CmdType::Middle => bind_thread_to_cpu(get_middle_group(), tid),
        CmdType::Background => bind_thread_to_cpu(get_background_group(), tid),
        CmdType::All => bind_thread_to_cpu(&[0, 1, 2, 3, 4, 5, 6, 7], tid),
    }
}

pub fn start_task(tid: &pid_t, thread_name: &str) {
    let thread_type = get_cmd_type(thread_name);
    execute_task(thread_type, tid);
}
