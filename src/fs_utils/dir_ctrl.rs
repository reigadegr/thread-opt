use std::fs;
const WORK_DIR: &str = "/dev/cpuset/thread-opt";

pub fn create_parent_dir() {
    match fs::create_dir(WORK_DIR) {
        Ok(_) => println!("{}目录创建成功", WORK_DIR),
        Err(e) => println!("{}目录创建失败：{}", WORK_DIR, e),
    }
    init_dir(WORK_DIR, "0-7");
}

pub fn create_work_space(cpus: &str) {
    let sub_dir = format!("{}/{}", WORK_DIR, cpus);
    match fs::create_dir(&sub_dir) {
        Ok(_) => println!("{}目录创建成功", sub_dir),
        Err(e) => println!("{}目录创建失败：{}", sub_dir, e),
    }
    init_dir(&sub_dir, cpus);
}

fn init_dir(path: &str, cpus: &str) {
    let cpus_path = format!("{}/cpus", path);
    fs::write(cpus_path, cpus.to_string());
    let path = format!("{}/mems", path);
    fs::write(path, "0".to_string());
}
