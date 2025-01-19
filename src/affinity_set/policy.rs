use crate::fs_utils::node_writer::node_writer;
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
    Backend,
}

pub fn get_cmd_type(thread_name: &str) -> CmdType {
    if TOP_THREADS.contains(&thread_name) {
        return CmdType::Top;
    }

    if MIDDLE_THREADS.contains(&thread_name) {
        return CmdType::Middle;
    }

    if BACKEND_THREADS.contains(&thread_name) {
        return CmdType::Backend;
    }

    CmdType::All
}

pub fn execute_task(cmd_type: CmdType, tid: &i32) -> anyhow::Result<()> {
    match cmd_type {
        CmdType::All => node_writer(get_top_dir(), tid),
        // CmdType::Top => println!("(tid: {}) executing Top task", tid),
        // CmdType::Middle => println!("(tid: {}) executing Middle task", tid),
        // CmdType::Backend => println!("(tid: {}) executing Backend task", tid),
        _ => node_writer(get_top_dir(), tid),
    };
    Ok(())
}
