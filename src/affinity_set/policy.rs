
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

pub fn execute_task(cmd_type: CmdType, tid: &i32) {
    match cmd_type {
        CmdType::All => println!("(tid: {}) executing All task", tid),
        CmdType::Top => println!("(tid: {}) executing Top task", tid),
        CmdType::Middle => println!("(tid: {}) executing Middle task", tid),
        CmdType::Backend => println!("(tid: {}) executing Backend task", tid),
    }
}
