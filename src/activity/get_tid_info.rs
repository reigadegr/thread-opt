use crate::utils::node_reader::read_to_byte;
use anyhow::{Result, anyhow};
use atoi::atoi;
use compact_str::CompactString;
use core::time::Duration;
use hashbrown::HashMap;
use libc::{closedir, opendir, pid_t, readdir};
use minstant::Instant;
use std::{ffi::CString, fs, path::Path};
extern crate alloc;
use alloc::format;

#[derive(Default)]
pub struct TidInfo {
    pub task_map: HashMap<pid_t, [u8; 16]>,
    pub tid_list: Vec<pid_t>,
    task_map_pid: pid_t,
    tid_list_pid: pid_t,
}

impl TidInfo {
    pub fn new() -> Self {
        Self::default()
    }
}

pub struct TidUtils {
    pub tid_info: TidInfo,
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

    pub fn get_task_map(&mut self, pid: pid_t) -> &HashMap<pid_t, [u8; 16]> {
        if self.last_refresh_task_map.elapsed() > Duration::from_millis(5000) {
            self.last_refresh_task_map = Instant::now();
            return &self.set_task_map(pid).task_map;
        }

        if self.tid_info.task_map_pid == pid {
            return &self.tid_info.task_map;
        }
        self.tid_info.task_map_pid = pid;

        &self.set_task_map(pid).task_map
    }

    pub fn get_tid_list(&mut self, pid: pid_t) -> &Vec<pid_t> {
        if self.last_refresh_tid_list.elapsed() > Duration::from_millis(5000) {
            self.last_refresh_tid_list = Instant::now();
            return &self.set_tid_list(pid).tid_list;
        }
        if self.tid_info.tid_list_pid == pid {
            return &self.tid_info.tid_list;
        }
        self.tid_info.tid_list_pid = pid;

        &self.set_tid_list(pid).tid_list
    }

    pub fn set_task_map(&mut self, pid: pid_t) -> &TidInfo {
        let Ok(tid_list) = read_task_dir(pid) else {
            return &self.tid_info;
        };

        let mut task_map: HashMap<pid_t, [u8; 16]> = HashMap::new();
        for tid in tid_list {
            let comm_path = format!("/proc/{tid}/comm");
            let Ok(comm) = read_to_byte(&comm_path) else {
                return &self.tid_info;
            };
            task_map.insert(tid, comm);
        }
        self.tid_info.task_map = task_map;
        &self.tid_info
    }

    pub fn set_tid_list(&mut self, pid: pid_t) -> &TidInfo {
        let Ok(tid_list) = read_task_dir(pid) else {
            self.tid_info.tid_list.clear();
            return &self.tid_info;
        };
        self.tid_info.tid_list = tid_list;
        &self.tid_info
    }
}

fn read_task_dir(pid: pid_t) -> Result<Vec<pid_t>> {
    let task_dir = format!("/proc/{pid}/task");
    let c_path = CString::new(task_dir).expect("Invalid path");

    // Open the directory
    let dir = unsafe { opendir(c_path.as_ptr()) };

    if dir.is_null() {
        return Err(anyhow!("Cannot read task_dir."));
    }

    // Read directory entries and filter out entries starting with '.'
    let entries: Vec<_> = unsafe {
        let dir_ptr = dir;
        std::iter::from_fn(move || {
            let entry = readdir(dir_ptr);
            if entry.is_null() {
                return None;
            }

            let d_name_ptr = (*entry).d_name.as_ptr();
            let bytes = std::slice::from_raw_parts(d_name_ptr, 7);

            if bytes.starts_with(b".") {
                Some(0)
            } else {
                Some(atoi::<i32>(bytes).expect("无法转换"))
            }
        })
        .filter(|&s| s != 0)
        .collect()
    };

    // Close the directory
    unsafe {
        closedir(dir);
    }

    Ok(entries)
}

pub fn get_process_name(pid: pid_t) -> Result<CompactString> {
    let cmdline = Path::new("/proc").join(pid.to_string()).join("cmdline");
    let cmdline = fs::read_to_string(cmdline)?;
    let cmdline = cmdline.split(':').next().unwrap_or_default();
    Ok(CompactString::new(cmdline.trim_matches(['\0']).trim()))
}
