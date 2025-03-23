use crate::utils::{
    guard::DirGuard,
    node_reader::{get_proc_path, read_to_byte},
};
use anyhow::{Result, anyhow};
use atoi::atoi;
use compact_str::CompactString;
use core::time::Duration;
use hashbrown::{HashMap, HashSet};
use libc::{DIR, opendir, pid_t, readdir, rewinddir};
use likely_stable::unlikely;
use minstant::Instant;
use stringzilla::sz;
extern crate alloc;
use alloc::vec::Vec;

#[derive(Default)]
pub struct TidInfo {
    pub task_map: HashMap<pid_t, [u8; 16]>,
    task_map_pid: pid_t,
}

impl TidInfo {
    pub fn new() -> Self {
        Self::default()
    }
}

pub struct TidUtils {
    pub tid_info: TidInfo,
    last_refresh_task_map: Instant,
}

impl TidUtils {
    pub fn new() -> Self {
        Self {
            tid_info: TidInfo::new(),
            last_refresh_task_map: Instant::now(),
        }
    }

    pub fn get_task_map(&mut self, pid: pid_t, dir_ptr: *mut DIR) -> &HashMap<pid_t, [u8; 16]> {
        if self.last_refresh_task_map.elapsed() > Duration::from_millis(3000) {
            self.last_refresh_task_map = Instant::now();
            return &self.set_task_map(dir_ptr).task_map;
        }

        if self.tid_info.task_map_pid == pid {
            return &self.tid_info.task_map;
        }
        self.tid_info.task_map_pid = pid;

        &self.set_task_map(dir_ptr).task_map
    }

    pub fn set_task_map(&mut self, dir_ptr: *mut DIR) -> &TidInfo {
        let tid_list = read_task_dir_cache(dir_ptr);

        #[cfg(debug_assertions)]
        let start = minstant::Instant::now();
        #[cfg(debug_assertions)]
        {
            let end = start.elapsed();
            log::debug!("转换HashSet时间: {:?}", end);
        }
        // self.tid_info
        // .task_map
        // .retain(|tid, _| tid_list.contains(tid));
        self.tid_info.task_map.clear();
        for tid in tid_list {
            // if self.tid_info.task_map.contains_key(&tid) {
            // continue;
            // }
            let comm_path = get_proc_path::<32, 5>(tid, b"/comm");
            let Ok(comm) = read_to_byte::<16>(&comm_path) else {
                continue;
            };
            self.tid_info.task_map.insert(tid, comm);
        }
        #[cfg(debug_assertions)]
        {
            let end = start.elapsed();
            log::debug!("读task_map时间: {:?}", end);
        }
        &self.tid_info
    }
}

pub fn read_task_dir(pid: pid_t) -> Result<HashSet<pid_t>> {
    let task_dir = get_proc_path::<32, 5>(pid, b"/task");

    let dir = unsafe { opendir(task_dir.as_ptr()) };
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
    let entries: HashSet<pid_t> = entries.into_iter().collect();
    Ok(entries)
}

pub fn read_task_dir_cache(dir_ptr: *mut DIR) -> HashSet<pid_t> {
    let entries: Vec<_> = unsafe {
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
    unsafe {
        rewinddir(dir_ptr);
    }

    let entries: HashSet<pid_t> = entries.into_iter().collect();
    entries
}

pub fn get_process_name(pid: pid_t) -> Result<CompactString> {
    let cmdline = get_proc_path::<32, 8>(pid, b"/cmdline");

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
    Ok(buffer)
}
