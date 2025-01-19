use anyhow::Result;
use std::time::Instant;
// use log::info;
use std::collections::HashMap;
use std::fs;
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
    last_refresh: Instant,
}

impl TidUtils {
    pub fn new() -> Self {
        Self {
            tid_info: TidInfo::new(),
            last_refresh: Instant::now(),
        }
    }

    pub fn get_task_map(&mut self, pid: &i32) -> &HashMap<i32, String> {
        if self.last_refresh.elapsed() > Duration::from_millis(5000) {
            self.last_refresh = Instant::now();
            return &self.set_task_map(pid).task_map;
        }

        let name = match get_process_name(pid) {
            Ok(name) => name,
            Err(_) => return &self.set_task_map(pid).task_map,
        };
        if self.tid_info.task_map_name == name {
            return &self.tid_info.task_map;
        }
        self.tid_info.task_map_name = name;
        &self.set_task_map(pid).task_map
    }

    pub fn get_tid_list(&mut self, pid: &i32) -> &Vec<i32> {
        if self.last_refresh.elapsed() > Duration::from_millis(5000) {
            self.last_refresh = Instant::now();
            return &self.set_tid_list(pid).tid_list;
        }
        let name = match get_process_name(pid) {
            Ok(name) => name,
            Err(_) => return &self.set_tid_list(pid).tid_list,
        };
        if self.tid_info.tid_list_name == name {
            return &self.tid_info.tid_list;
        }
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
            let file_name = match entry {
                Ok(name) => name.file_name(),
                Err(_) => return &self.tid_info,
            };

            if let Some(tid) = file_name.to_str() {
                if tid.starts_with('.') {
                    continue;
                }
                let comm_path = format!("/proc/{}/task/{}/comm", pid, tid);
                let comm = match read_file(Path::new(&comm_path)) {
                    Ok(comm) => comm,
                    Err(_) => return &self.tid_info,
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
            let file_name = match entry {
                Ok(name) => name.file_name(),
                Err(_) => return &self.tid_info,
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

pub fn read_file(file: &Path) -> Result<String> {
    let s = fs::read_to_string(file)?;
    Ok(s.trim().to_string())
}

pub fn get_process_name(pid: &i32) -> Result<String> {
    let cmdline = Path::new("/proc").join(pid.to_string()).join("cmdline");
    let cmdline = fs::read_to_string(cmdline)?;
    let cmdline = cmdline.split(':').next().unwrap_or_default();
    Ok(cmdline.trim_matches(['\0']).trim().to_string())
}
