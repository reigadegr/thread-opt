use crate::activity::get_top_app::TopAppUtils;
use anyhow::Result;
use log::info;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::time::Duration;

#[derive(Default)]
pub struct TidInfo {
    task_map: HashMap<i32, String>,
    tid_list: Vec<i32>,
}

impl TidInfo {
    pub fn new() -> Self {
        Self::default()
    }
}

pub struct TidUtils {
    tid_info: TidInfo,
    top_app_utils: TopAppUtils,
}

impl TidUtils {
    pub fn new() -> Self {
        Self {
            tid_info: TidInfo::new(),
            top_app_utils: TopAppUtils::new(),
        }
    }

    pub fn get_task_map(&mut self, pid: &i32) -> &HashMap<i32, String> {
        &self.set_task_map(pid).task_map
    }

    pub fn set_task_map(&mut self, pid: &i32) -> &TidInfo {
        let task_dir = format!("/proc/{}/task", pid);
        let mut task_map = HashMap::new();

        let dp = match fs::read_dir(&task_dir) {
            Ok(dir) => dir,
            Err(_) => {
                info!("读目录出错");
                return &self.tid_info;
            }
        };

        for entry in dp {
            let file_name = loop {
                match entry {
                    Ok(name) => break name.file_name(),
                    Err(_) => {
                        info!("读获取文件名出错");
                        std::thread::sleep(Duration::from_millis(1000));
                    }
                };
            };

            if let Some(tid) = file_name.to_str() {
                if tid.starts_with('.') {
                    continue;
                }
                let comm_path = format!("/proc/{}/task/{}/comm", pid, tid);
                let comm = loop {
                    match read_file(Path::new(&comm_path)) {
                        Ok(comm) => break comm,
                        Err(_) => {
                            info!("读comm出错");
                            std::thread::sleep(Duration::from_millis(1000));
                        }
                    };
                };
                let tid = tid.parse::<i32>();
                match tid {
                    Ok(tid) => task_map.insert(tid, comm),
                    Err(_) => continue,
                };
            };
        }
        self.tid_info.task_map = task_map;
        &self.tid_info
    }
}

fn read_file(file: &Path) -> Result<String> {
    let mut s = String::new();
    File::open(file)?.read_to_string(&mut s)?;
    Ok(s.trim().to_string())
}

pub fn get_tid_list(pid: &i32) -> Result<Vec<i32>> {
    let task_dir = format!("/proc/{}/task", pid);
    let mut tid_list = Vec::new();

    let dp = fs::read_dir(task_dir)?;
    for entry in dp {
        let file_name = entry?.file_name();
        if let Some(tid) = file_name.to_str() {
            if tid.starts_with('.') {
                continue;
            }
            let tid = tid.parse::<i32>();
            match tid {
                Ok(tid) => tid_list.push(tid),
                Err(_) => continue,
            }
        }
    }
    Ok(tid_list)
}
