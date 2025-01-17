use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
fn read_file(file: &Path) -> Result<String> {
    let mut s = String::new();
    File::open(file)?.read_to_string(&mut s)?;
    Ok(s.trim().to_string()) // 返回trim后的字符串
}

pub fn get_task_map(pid: &str) -> Result<HashMap<i32, String>> {
    // println!("开始获取map,pid={}", pid);
    let task_dir = format!("/proc/{}/task", pid);
    let mut task_map = HashMap::new();

    let dp = fs::read_dir(&task_dir)?;
    for entry in dp {
        // let entry = entry?;
        let file_name = entry?.file_name();
        if let Some(tid) = file_name.to_str() {
            if tid.starts_with('.') {
                continue;
            }
            let comm_path = format!("/proc/{}/task/{}/comm", pid, tid);
            let comm = read_file(Path::new(&comm_path))?;
            let tid = tid.parse::<i32>();
            match tid {
                Ok(tid) => task_map.insert(tid, comm),
                Err(_) => continue,
            };
        };
    }

    Ok(task_map)
}

pub fn get_tid_list(pid: &str) -> Result<Vec<i32>> {
    // println!("开始获取map,pid={}", pid);
    let task_dir = format!("/proc/{}/task", pid);
    let mut tid_list = Vec::new();

    let dp = fs::read_dir(task_dir)?;
    for entry in dp {
        // let entry = entry?;
        let file_name = entry?.file_name();
        if let Some(tid) = file_name.to_str() {
            if tid.starts_with('.') {
                continue;
            }
            // let tid = tid.parse::<i32>().unwrap_or(0);
            let tid = tid.parse::<i32>();
            match tid {
                Ok(tid) => tid_list.push(tid),
                Err(_) => continue,
            }
        }
    }
    Ok(tid_list)
}
