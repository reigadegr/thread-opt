use super::dir_ctrl::WORK_DIR;
use anyhow::Result;
use std::fs;
pub fn write_node(path: Result<String>, value: &u32) {
    let path = match path {
        Ok(path) => path,
        Err(_) => WORK_DIR.to_string(),
    };
    let node = format!("{}/tasks", path);
    let _ = fs::write(node, value.to_string());
}

pub fn write_node_origin(path: &str, value: &u32) {
    let node = format!("{}/tasks", path);
    let _ = fs::write(node, value.to_string());
}
