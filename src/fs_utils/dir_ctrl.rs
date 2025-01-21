use anyhow::{Result, anyhow};
use log::info;
use once_cell::sync::OnceCell;
use std::fs;
use std::path::Path;

pub const WORK_DIR: &str = "/dev/cpuset/thread-opt";

static TOP_DIR: OnceCell<String> = OnceCell::new();

static MIDDLE_DIR: OnceCell<String> = OnceCell::new();

static BACKEND_DIR: OnceCell<String> = OnceCell::new();

fn basename(path: &str) -> String {
    let file_name = Path::new(path)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("0-7");
    file_name.to_string()
}

pub fn middle_dir_ctrl() -> Result<()> {
    let top_dir = get_top_dir()?;
    let background_dir = get_background_dir()?;

    // 重新声明 top_dir 和 background_dir，提取文件名（basename）
    let top_dir = basename(&top_dir);

    let background_dir = basename(&background_dir);

    // 分割 top_dir 和 background_dir
    let top_dir_parts: Vec<&str> = top_dir.split('-').collect();
    let background_dir_parts: Vec<&str> = background_dir.split('-').collect();

    // 提取数字并转换为 i32
    let top_dir_a: i32 = top_dir_parts[0].parse()?;
    let background_dir_b: i32 = background_dir_parts[1].parse()?;

    if top_dir_a - background_dir_b > 1 {
        info!("集群数量大于或等于三个，需要创建Middle目录");
        let cpus = format!("{}-{}", background_dir_b + 1, top_dir_a - 1);
        let sub_dir = format!("{}/{}", WORK_DIR, cpus);
        init_node(&sub_dir, &cpus);
        MIDDLE_DIR.get_or_init(|| sub_dir.clone());
    } else {
        let background_dir = get_background_dir()?;
        MIDDLE_DIR.get_or_init(|| background_dir);
    }
    Ok(())
}

pub fn get_top_dir() -> Result<String> {
    TOP_DIR.get().ok_or(anyhow!("TOP_DIR not found")).cloned()
}

pub fn get_middle_dir() -> Result<String> {
    MIDDLE_DIR
        .get()
        .ok_or(anyhow!("MIDDLE_DIR not found"))
        .cloned()
}

pub fn get_background_dir() -> Result<String> {
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
    init_node(WORK_DIR, "0-7");
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
    init_node(&sub_dir, cpus);
}

fn init_node(path: &str, cpus: &str) {
    let cpus_path = format!("{}/cpus", path);
    let _ = fs::write(cpus_path, cpus);
    let path = format!("{}/mems", path);
    let _ = fs::write(path, "0");
}
