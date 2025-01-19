use anyhow::{Result, anyhow};
use log::info;
use once_cell::sync::OnceCell;
use std::fs;

const WORK_DIR: &str = "/dev/cpuset/thread-opt";

const GLOBAL_NODE: &str = "/dev/cpuset/thread-opt/tasks";

static TOP_DIR: OnceCell<String> = OnceCell::new();

static BACKEND_DIR: OnceCell<String> = OnceCell::new();

pub fn get_top_dir() -> Result<String> {
    TOP_DIR.get().ok_or(anyhow!("TOP_DIR not found")).cloned()
}

pub fn get_backend_dir() -> Result<String> {
    BACKEND_DIR
        .get()
        .ok_or(anyhow!("BACKEND_DIR not found"))
        .cloned()
}

pub fn create_parent_dir() {
    match fs::create_dir(WORK_DIR) {
        Ok(_) => info!("{}目录创建成功", WORK_DIR),
        Err(e) => info!("{}目录创建失败：{}", WORK_DIR, e),
    }
    init_dir(WORK_DIR, "0-7");
}

pub fn create_sub_work_space(cpus: &str) {
    let sub_dir = format!("{}/{}", WORK_DIR, cpus);

    if sub_dir.contains("7") {
        TOP_DIR.get_or_init(|| sub_dir.clone());
    }

    if sub_dir.contains("0") {
        BACKEND_DIR.get_or_init(|| sub_dir.clone());
    }

    match fs::create_dir(&sub_dir) {
        Ok(_) => info!("{}目录创建成功", sub_dir),
        Err(e) => info!("{}目录创建失败：{}", sub_dir, e),
    }
    init_dir(&sub_dir, cpus);
}

fn init_dir(path: &str, cpus: &str) {
    let cpus_path = format!("{}/cpus", path);
    fs::write(cpus_path, cpus);
    let path = format!("{}/mems", path);
    fs::write(path, "0");
}
