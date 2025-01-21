use crate::fs_utils::dir_ctrl::WORK_DIR;
use crate::fs_utils::dir_ctrl::get_background_dir;
use crate::fs_utils::dir_ctrl::get_middle_dir;
use crate::fs_utils::dir_ctrl::get_top_dir;
use crate::fs_utils::node_writer::write_node;
use crate::fs_utils::node_writer::write_node_origin;

const TOP_THREADS: [&str; 3] = ["GameThread", "RHIThread", "UnityMain"];
const MIDDLE_THREADS: [&str; 1] = ["UnityGfxDeviceW"];
const BACKEND_THREADS: [&str; 0] = [];
const ALL_THREADS: [&str; 0] = [];
const MIDDLE_REGEX_THREADS: [&str; 3] = ["Thread-", "Job.Worker", "RenderThread"];

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
    for prev_name in MIDDLE_REGEX_THREADS {
        if thread_name.starts_with(prev_name) {
            return CmdType::Middle;
        }
    }
    CmdType::Middle
}

fn execute_task(cmd_type: CmdType, tid: &i32) {
    match cmd_type {
        CmdType::Top => write_node(get_top_dir(), tid),
        CmdType::Middle => write_node(get_middle_dir(), tid),
        CmdType::Background => write_node(get_background_dir(), tid),
        CmdType::All => write_node_origin(WORK_DIR, tid),
    }
}

pub fn start_task(tid: &i32, thread_name: &str) {
    let thread_type = get_cmd_type(thread_name);
    execute_task(thread_type, tid);
}
