use crate::fs_utils::{
    dir_ctrl::{get_background_dir, get_middle_dir, get_top_dir, WORK_DIR},
    node_writer::{write_node, write_node_origin},
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
        CmdType::Top => write_node(get_top_dir(), tid),
        CmdType::Middle => write_node(get_middle_dir(), tid),
        CmdType::Background => write_node(get_background_dir(), tid),
        CmdType::All => write_node_origin(WORK_DIR, tid),
    }
}

pub fn start_task(tid: &pid_t, thread_name: &str) {
    let thread_type = get_cmd_type(thread_name);
    execute_task(thread_type, tid);
}
