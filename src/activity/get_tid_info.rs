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
    task_map_name: String,
    tid_list_name: String,
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
}

impl TidUtils {
    pub fn new() -> Self {
        Self {
            tid_info: TidInfo::new(),
        }
    }

    pub fn get_task_map(&mut self, pid: &i32) -> &HashMap<i32, String> {
        let name = match get_process_name(pid) {
            Ok(name) => name,
            Err(_) => return &self.set_task_map(pid).task_map,
        };
        if self.tid_info.task_map_name == name {
            info!("使用缓存");
            return &self.tid_info.task_map;
        }
        info!("不使用缓存");
        self.tid_info.task_map_name = name;
        &self.set_task_map(pid).task_map
    }

    pub fn get_tid_list(&mut self, pid: &i32) -> &Vec<i32> {
        let name = match get_process_name(pid) {
            Ok(name) => name,
            Err(_) => return &self.set_tid_list(pid).tid_list,
        };
        if self.tid_info.tid_list_name == name {
            info!("使用缓存tidlist");
            return &self.tid_info.tid_list;
        }
        info!("不使用缓存tidlist");
        self.tid_info.tid_list_name = name;
        &self.set_tid_list(pid).tid_list
    }

    pub fn set_task_map(&mut self, pid: &i32) -> &TidInfo {
        let task_dir = format!("/proc/{}/task", pid);
        let mut task_map = HashMap::new();

        let dp = match fs::read_dir(&task_dir) {
            Ok(dir) => dir,
            Err(_) => return &self.tid_info,
        };

        for entry in dp {
            let file_name = loop {
                match entry {
                    Ok(name) => break name.file_name(),
                    Err(_) => std::thread::sleep(Duration::from_millis(1000)),
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
                        Err(_) => std::thread::sleep(Duration::from_millis(1000)),
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

    pub fn set_tid_list(&mut self, pid: &i32) -> &TidInfo {
        let task_dir = format!("/proc/{}/task", pid);
        let mut tid_list = Vec::new();

        let dp = match fs::read_dir(&task_dir) {
            Ok(dir) => dir,
            Err(_) => return &self.tid_info,
        };
        for entry in dp {
            let file_name = loop {
                match entry {
                    Ok(name) => break name.file_name(),
                    Err(_) => std::thread::sleep(Duration::from_millis(1000)),
                };
            };
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
        self.tid_info.tid_list = tid_list;
        &self.tid_info
    }
}

fn read_file(file: &Path) -> Result<String> {
    let mut s = String::new();
    File::open(file)?.read_to_string(&mut s)?;
    Ok(s.trim().to_string())
}

pub fn get_process_name(pid: &i32) -> anyhow::Result<String> {
    let cmdline = Path::new("/proc").join(pid.to_string()).join("cmdline");
    let cmdline = fs::read_to_string(cmdline)?;
    let cmdline = cmdline.split(':').next().unwrap_or_default();
    Ok(cmdline.trim_matches(['\0']).trim().to_string())
}
