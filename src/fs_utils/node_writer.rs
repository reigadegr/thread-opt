use super::dir_ctrl::WORK_DIR;
use anyhow::Result;
use libc::pid_t;
use std::fs;
pub fn write_node(path: Result<String>, value: &pid_t) {
    let path = path.unwrap_or_else(|_| WORK_DIR.to_string());
    let node = format!("{}/tasks", path);
    let _ = fs::write(node, value.to_string());
}

pub fn write_node_origin(path: &str, value: &pid_t) {
    let node = format!("{}/tasks", path);
    let _ = fs::write(node, value.to_string());
}
