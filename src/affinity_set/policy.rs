use crate::fs_utils::dir_ctrl::WORK_DIR;
use crate::fs_utils::node_writer::write_node;
use crate::fs_utils::node_writer::write_node_origin;
use crate::get_background_dir;
use crate::get_middle_dir;
use crate::get_top_dir;

const TOP_THREADS: [&str; 2] = ["Unity", "UnityMain"];
const MIDDLE_THREADS: [&str; 1] = ["RenderThread"];
const BACKEND_THREADS: [&str; 0] = [];
const ALL_THREADS: [&str; 0] = [];

#[derive(Debug)]
pub enum CmdType {
    All,
    Top,
    Middle,
    Background,
}

pub fn get_cmd_type(thread_name: &str) -> CmdType {
    if TOP_THREADS.contains(&thread_name) {
        return CmdType::Top;
    }

    if MIDDLE_THREADS.contains(&thread_name) {
        return CmdType::Middle;
    }

    if BACKEND_THREADS.contains(&thread_name) {
        return CmdType::Background;
    }

    CmdType::All
}

pub fn execute_task(cmd_type: CmdType, tid: &i32) {
    match cmd_type {
        CmdType::Top => write_node(get_top_dir(), tid),
        CmdType::Middle => write_node(get_middle_dir(), tid),
        CmdType::Background => write_node(get_background_dir(), tid),
        _ => write_node_origin(WORK_DIR, tid),
    }
}
