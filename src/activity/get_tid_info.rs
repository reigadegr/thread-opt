use crate::utils::node_reader::{get_proc_path, read_to_byte};
use anyhow::{Result, anyhow};
use atoi::atoi;
use compact_str::CompactString;
use core::time::Duration;
use log::warn;
use minstant::Instant;
use rustix::fs::{self, CWD, Mode, OFlags};
use std::{
    collections::{
        HashMap, HashSet,
        hash_map::Entry::{Occupied, Vacant},
    },
    ffi::OsStr,
    fs::File,
    io::{ErrorKind, Read, Seek, SeekFrom},
    os::unix::ffi::OsStrExt,
};
use stringzilla::sz;

#[derive(Debug)]
pub struct FileCache {
    files: HashMap<i32, File>,
}

impl FileCache {
    fn new() -> Self {
        Self {
            files: HashMap::new(),
        }
    }

    fn read_with_cache<const N: usize>(&mut self, tid: i32) -> Result<[u8; N]> {
        let file = match self.files.entry(tid) {
            Vacant(e) => {
                let path = get_proc_path::<32>(tid, b"/comm");
                let end = sz::find(path, b"\0").unwrap_or(path.len());
                let path_str = &path[..end];
                let path_str = OsStr::from_bytes(path_str);
                let file = File::open(path_str).map_err(|e| anyhow!("Cannot open file: {e}"))?;
                e.insert(file)
            }
            Occupied(e) => e.into_mut(),
        };

        if let Err(e) = file.seek(SeekFrom::Start(0)) {
            self.files.remove(&tid);
            return Err(e.into());
        }

        let mut buffer = [0u8; N];
        match file.read_exact(&mut buffer) {
            Ok(()) => Ok(buffer),
            Err(e) if e.kind() == ErrorKind::UnexpectedEof => Ok(buffer),
            Err(e) => {
                self.files.remove(&tid);
                Err(e.into())
            }
        }
    }

    pub fn clear(&mut self) {
        self.files.clear();
    }
}

#[derive(Default)]
pub struct TidInfo {
    pub task_map: HashMap<i32, [u8; 16]>,
    task_map_pid: i32,
}

impl TidInfo {
    pub fn new() -> Self {
        Self::default()
    }
}

pub struct TidUtils {
    pub tid_info: TidInfo,
    last_refresh_task_map: Instant,
    pub file_cache: FileCache,
}

impl TidUtils {
    pub fn new() -> Self {
        Self {
            tid_info: TidInfo::new(),
            last_refresh_task_map: Instant::now(),
            file_cache: FileCache::new(),
        }
    }

    pub fn get_task_map(&mut self, pid: i32) -> &HashMap<i32, [u8; 16]> {
        if self.last_refresh_task_map.elapsed() > Duration::from_millis(3000) {
            self.last_refresh_task_map = Instant::now();
            return &self.set_task_map(pid).task_map;
        }

        if self.tid_info.task_map_pid == pid {
            return &self.tid_info.task_map;
        }
        self.tid_info.task_map_pid = pid;

        &self.set_task_map(pid).task_map
    }

    pub fn set_task_map(&mut self, pid: i32) -> &TidInfo {
        let tid_list = match read_task_dir(pid) {
            Ok(list) => list,
            Err(e) => {
                warn!("Failed to read task directory for pid {pid}: {e}");
                self.tid_info.task_map.clear();
                return &self.tid_info;
            }
        };

        #[cfg(debug_assertions)]
        let start = minstant::Instant::now();
        self.tid_info.task_map.clear();
        for tid in tid_list {
            let Ok(comm) = self.file_cache.read_with_cache::<16>(tid) else {
                continue;
            };
            self.tid_info.task_map.insert(tid, comm);
        }
        #[cfg(debug_assertions)]
        {
            let end = start.elapsed();
            log::debug!("读task_map时间: {end:?}");
        }
        &self.tid_info
    }
}

pub fn read_task_dir(pid: i32) -> Result<HashSet<i32>> {
    let task_dir = get_proc_path::<32>(pid, b"/task");
    let end = sz::find(task_dir, b"\0").unwrap_or(task_dir.len());
    let path_slice = &task_dir[..end];

    let path = OsStr::from_bytes(path_slice);

    let fd = fs::openat(
        CWD,
        path,
        OFlags::RDONLY | OFlags::DIRECTORY | OFlags::CLOEXEC,
        Mode::empty(),
    )
    .map_err(|e| anyhow!("Failed to open task dir: {e}"))?;

    let mut dir =
        fs::Dir::read_from(fd).map_err(|e| anyhow!("Failed to create dir stream: {e}"))?;

    let mut entries = HashSet::new();

    loop {
        match dir.next() {
            None => break,
            Some(Ok(entry)) => {
                let name = entry.file_name().to_bytes();

                if name.starts_with(b".") {
                    continue;
                }

                if let Some(tid) = atoi::<i32>(name) {
                    entries.insert(tid);
                }
            }
            Some(Err(_)) => {}
        }
    }
    Ok(entries)
}

pub fn get_process_name(pid: i32) -> Result<CompactString> {
    let cmdline = get_proc_path::<32>(pid, b"/cmdline");

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
