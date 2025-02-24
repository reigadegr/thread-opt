use crate::utils::{guard::DirGuard, node_reader::read_to_byte};
use anyhow::{Result, anyhow};
use atoi::atoi;
use compact_str::CompactString;
use core::time::Duration;
use hashbrown::HashMap;
use libc::{opendir, pid_t, readdir};
use likely_stable::unlikely;
use minstant::Instant;
use stringzilla::sz;
extern crate alloc;
use alloc::{ffi::CString, format};

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
            let Ok(comm) = read_to_byte::<16>(&comm_path) else {
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
    let c_path = CString::new(task_dir)?;

    let dir = unsafe { opendir(c_path.as_ptr()) };
    if unlikely(dir.is_null()) {
        return Err(anyhow!("Cannot read task_dir."));
    }
    let _dir_ptr_guard = DirGuard::new(dir);
    let entries: Vec<_> = unsafe {
        let dir_ptr = dir;

        core::iter::from_fn(move || {
            let entry = readdir(dir_ptr);
            if unlikely(entry.is_null()) {
                return None;
            }

            let d_name_ptr = (*entry).d_name.as_ptr();
            // 这里，d_name_ptr长度不可能超过6,Linux PID最大32768
            let bytes = core::slice::from_raw_parts(d_name_ptr, 6);
            // 如果以'.'开头，会被fallback为0，最后被过滤
            Some(atoi::<pid_t>(bytes).unwrap_or(0))
        })
        .filter(|&s| s != 0)
        .collect()
    };
    Ok(entries)
}

pub fn get_process_name(pid: pid_t) -> Result<CompactString> {
    let cmdline = format!("/proc/{pid}/cmdline");
    let buffer = read_to_byte::<128>(&cmdline)?;
    let pos = sz::find(buffer, b":");
    if let Some(sub) = pos {
        let buffer = &buffer[..sub];
        let buffer = CompactString::from_utf8(buffer)?;
        return Ok(buffer);
    }

    let pos = sz::find(buffer, b"\0");
    let buffer = pos.map_or(&buffer[..], |pos| &buffer[..pos]);

    let buffer = CompactString::from_utf8(buffer)?;
    println!("经计算后的包名:{buffer}");
    Ok(buffer)
}
