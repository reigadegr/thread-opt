use super::dir_ctrl::WORK_DIR;
use anyhow::Result;
use std::fs;
pub fn node_writer(path: Result<String>, value: &i32) -> Result<()> {
    let path = match path {
        Ok(path) => path,
        Err(_) => WORK_DIR.to_string(),
    };
    let node = format!("{}/tasks", path);
    fs::write(node, value.to_string());
    Ok(())
}
