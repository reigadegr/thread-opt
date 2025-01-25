use crate::{
    affinity_utils::bind_thread_to_cpu,
    cgroup::group_info::{get_background_group, get_middle_group, get_top_group},
};
use libc::pid_t;

const TOP: [&str; 1] = ["GameThread"];
const ONLY6: [&str; 2] = ["RHIThread", "RenderThread"];
const BACKEND: [&str; 0] = [];
const MIDDLE: [&str; 0] = [];

enum CmdType {
    Top,
    Middle,
    Background,
    Only6,
}

fn get_cmd_type(thread_name: &str) -> CmdType {
    // 使用 starts_with 方法匹配线程
    for prev_name in TOP {
        if thread_name.starts_with(prev_name) {
            return CmdType::Top;
        }
    }

    for prev_name in MIDDLE {
        if thread_name.starts_with(prev_name) {
            return CmdType::Middle;
        }
    }

    for prev_name in BACKEND {
        if thread_name.starts_with(prev_name) {
            return CmdType::Background;
        }
    }

    for prev_name in ONLY6 {
        if thread_name.starts_with(prev_name) {
            return CmdType::Only6;
        }
    }

    CmdType::Middle
}

fn execute_task(cmd_type: &CmdType, tid: pid_t) {
    match cmd_type {
        CmdType::Top => {
            let top_group = get_top_group();
            if top_group == [6, 7] {
                bind_thread_to_cpu(&[7], tid);
                return;
            }
            bind_thread_to_cpu(get_top_group(), tid);
        }
        CmdType::Only6 => {
            let top_group = get_top_group();
            if top_group == [6, 7] {
                bind_thread_to_cpu(&[6], tid);
                return;
            }
            bind_thread_to_cpu(get_middle_group(), tid);
        }
        CmdType::Middle => bind_thread_to_cpu(get_middle_group(), tid),
        CmdType::Background => bind_thread_to_cpu(get_background_group(), tid),
    }
}

pub fn start_task(tid: pid_t, thread_name: &str) {
    let thread_type = get_cmd_type(thread_name);
    execute_task(&thread_type, tid);
}
