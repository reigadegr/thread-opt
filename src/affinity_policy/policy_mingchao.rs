use crate::{
    affinity_utils::bind_thread_to_cpu,
    cgroup::group_info::{get_background_group, get_middle_group, get_top_group},
};
use libc::pid_t;

const TOP_THREADS: [&str; 1] = ["GameThread"];
const MIDDLE_THREADS: [&str; 1] = ["RHIThread"];
const BACKEND_THREADS: [&str; 0] = [];
const MIDDLE_REGEX_THREADS: [&str; 1] = ["RenderThread"];

enum CmdType {
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

    // 使用 starts_with 方法匹配线程
    for prev_name in MIDDLE_REGEX_THREADS {
        if thread_name.starts_with(prev_name) {
            return CmdType::Middle;
        }
    }
    CmdType::Middle
}

fn execute_task(cmd_type: &CmdType, tid: pid_t, thread_name: &str) {
    let top_group = get_top_group();
    match cmd_type {
        CmdType::Top => {
            if thread_name == "GameThread" && top_group == [6, 7] {
                bind_thread_to_cpu(&[7], tid);
                return;
            }
            bind_thread_to_cpu(get_top_group(), tid);
        }
        CmdType::Middle => {
            if (thread_name == "RHIThread" || thread_name.starts_with("RenderThread"))
                && top_group == [6, 7]
            {
                bind_thread_to_cpu(&[6], tid);
                return;
            }
            bind_thread_to_cpu(get_middle_group(), tid);
        }
        CmdType::Background => bind_thread_to_cpu(get_background_group(), tid),
    }
}

pub fn start_task(tid: pid_t, thread_name: &str) {
    let thread_type = get_cmd_type(thread_name);
    execute_task(&thread_type, tid, thread_name);
}
