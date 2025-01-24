use crate::fs_utils::node_reader::read_file;
use anyhow::Result;
use libc::pid_t;
use log::info;
use std::{
    collections::HashMap,
    fs,
    path::Path,
    time::{Duration, Instant},
};

#[derive(Default)]
pub struct TidInfo {
    task_map_name: String,
    tid_list_name: String,
    task_map: HashMap<pid_t, String>,
    tid_list: Vec<pid_t>,
}

impl TidInfo {
    pub fn new() -> Self {
        Self::default()
    }
}

pub struct TidUtils {
    tid_info: TidInfo,
    last_refresh_task_map: Instant,
    last_refresh_tid_list: Instant,
}

impl TidUtils {
    pub fn new() -> Self {
        Self {
            tid_info: TidInfo::new(),
            last_refresh_task_map: Instant::now(),
            last_refresh_tid_list: Instant::now(),
        }
    }

    pub fn get_task_map(&mut self, pid: &pid_t) -> &HashMap<pid_t, String> {
        if self.last_refresh_task_map.elapsed() > Duration::from_millis(5000) {
            self.last_refresh_task_map = Instant::now();
            return &self.set_task_map(pid).task_map;
        }

        if let Ok(name) = get_process_name(pid) {
            if self.tid_info.task_map_name == name {
                return &self.tid_info.task_map;
            }
            self.tid_info.task_map_name = name;
        }

        &self.set_task_map(pid).task_map
    }

    pub fn get_tid_list(&mut self, pid: &pid_t) -> &Vec<pid_t> {
        if self.last_refresh_tid_list.elapsed() > Duration::from_millis(5000) {
            self.last_refresh_tid_list = Instant::now();
            return &self.set_tid_list(pid).tid_list;
        }

        if let Ok(name) = get_process_name(pid) {
            if self.tid_info.tid_list_name == name {
                return &self.tid_info.tid_list;
            }
            self.tid_info.tid_list_name = name;
        }

        &self.set_tid_list(pid).tid_list
    }

    fn read_task_dir(&self, pid: &pid_t) -> Result<Vec<pid_t>> {
        let task_dir = format!("/proc/{pid}/task");
        let tid_list = fs::read_dir(task_dir)?
            .filter_map(|entry| {
                entry
                    .ok()
                    .and_then(|e| e.file_name().to_string_lossy().parse::<pid_t>().ok())
            })
            .collect();
        Ok(tid_list)
    }

    pub fn set_task_map(&mut self, pid: &pid_t) -> &TidInfo {
        let tid_list = match self.read_task_dir(pid) {
            Ok(list) => list,
            Err(e) => {
                info!("Failed to read task directory: {}", e);
                return &self.tid_info;
            }
        };

        let mut task_map: HashMap<pid_t, String> = HashMap::new();
        for tid in tid_list.iter() {
            let comm_path = format!("/proc/{tid}/comm");
            let comm = match read_file(Path::new(&comm_path)) {
                Ok(comm) => comm,
                Err(e) => {
                    info!("Failed to read comm file for tid {}: {}", tid, e);
                    return &self.tid_info;
                }
            };
            task_map.insert(*tid, comm);
        }
        self.tid_info.task_map = task_map;
        &self.tid_info
    }

    pub fn set_tid_list(&mut self, pid: &pid_t) -> &TidInfo {
        let tid_list = match self.read_task_dir(pid) {
            Ok(list) => list,
            Err(e) => {
                info!("Failed to read task directory: {}", e);
                return &self.tid_info;
            }
        };
        self.tid_info.tid_list = tid_list;
        &self.tid_info
    }
}

pub fn get_process_name(pid: &pid_t) -> Result<String> {
    let cmdline = Path::new("/proc").join(pid.to_string()).join("cmdline");
    let cmdline = fs::read_to_string(cmdline)?;
    let cmdline = cmdline.split(':').next().unwrap_or_default();
    Ok(cmdline.trim_matches(['\0']).trim().to_string())
}
