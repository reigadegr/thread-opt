use super::group_info::{get_background_group, get_top_group};
use crate::utils::{guard::DirGuard, node_reader::read_file};
use anyhow::{Result, anyhow};
use compact_str::CompactString;
use libc::{DT_DIR, opendir, readdir};
use likely_stable::{likely, unlikely};
use log::info;
use stringzilla::sz;
extern crate alloc;
use alloc::{boxed::Box, vec::Vec};

pub static TOP_GROUP: std::sync::LazyLock<Box<[u8]>> = std::sync::LazyLock::new(|| {
    let cores = analysis_cgroup_new("7");
    if let Ok(cores) = cores {
        if *cores == [4, 5, 6, 7] {
            return Box::new([7]);
        }
        return cores;
    }
    Box::new([6])
});

pub static BACKEND_GROUP: std::sync::LazyLock<Box<[u8]>> =
    std::sync::LazyLock::new(|| analysis_cgroup_new("0").unwrap());

pub static MIDDLE_GROUP: std::sync::LazyLock<Box<[u8]>> = std::sync::LazyLock::new(|| {
    let mut all_core =
        unsafe { heapless::Vec::<u8, 8>::from_slice(&[0, 1, 2, 3, 4, 5, 6, 7]).unwrap_unchecked() };
    let background_values = get_background_group();
    let top_values = get_top_group();

    if top_values == [6] {
        all_core.remove(7);
    }

    for &value in background_values.iter().chain(top_values.iter()) {
        all_core.retain(|&x| x != value);
    }

    if all_core.is_empty() {
        info!("MIDDLE_GROUP initializing with BACKEND_GROUP.");
        background_values.into()
    } else {
        // 否则，使用处理后的 all_core 初始化 MIDDLE_GROUP
        all_core.as_slice().into()
    }
});

fn read_cgroup_dir() -> Result<Vec<[u8; 64]>> {
    let cgroup = b"/sys/devices/system/cpu/cpufreq\0";
    let dir = unsafe { opendir(cgroup.as_ptr()) };

    if unlikely(dir.is_null()) {
        return Err(anyhow!("Cannot read task_dir."));
    }
    let _dir_ptr_guard = DirGuard::new(dir);
    let mut entries = Vec::new();
    unsafe {
        let dir_ptr = dir;
        loop {
            let entry = readdir(dir_ptr);
            if unlikely(entry.is_null()) {
                break;
            }

            if unlikely((*entry).d_type != DT_DIR) {
                continue;
            }

            // 获取目录项的名称
            let d_name_ptr = (*entry).d_name.as_ptr();

            let d_bytes = core::slice::from_raw_parts(d_name_ptr, 7);

            if d_bytes.first() == Some(&b'.') {
                continue;
            }

            let mut real_path = [0u8; 64];
            real_path[..=30].copy_from_slice(&cgroup[..=30]);
            real_path[31] = b'/';
            real_path[32..=38].copy_from_slice(d_bytes);
            entries.push(real_path);
        }
    }
    Ok(entries)
}

pub fn analysis_cgroup_new(target_core: &str) -> Result<Box<[u8]>> {
    let entries = read_cgroup_dir()?;
    for entry in entries {
        let core_dir_ptr = unsafe { opendir(entry.as_ptr()) };

        if unlikely(core_dir_ptr.is_null()) {
            continue;
        }

        let _dir_ptr_guard = DirGuard::new(core_dir_ptr);

        unsafe {
            let dir_ptr = core_dir_ptr;

            loop {
                let entry_ptr = readdir(dir_ptr);
                if unlikely(entry_ptr.is_null()) {
                    break;
                }
                let d_name_ptr = (*entry_ptr).d_name.as_ptr();
                // 这里，最大为related_cpus的长度，12
                let bytes = core::slice::from_raw_parts(d_name_ptr, 12);

                if likely(sz::find(bytes, b"related_cpus").is_none()) {
                    continue;
                }
                let mut real_path = [0u8; 64];
                real_path[..=38].copy_from_slice(&entry[..=38]);
                real_path[39] = b'/';
                real_path[40..52].copy_from_slice(&b"related_cpus"[..]);
                let content =
                    read_file::<16>(&real_path).unwrap_or_else(|_| CompactString::new("8"));
                // 解析文件内容
                let nums: Vec<&str> = content.split_whitespace().collect();
                let rs = init_group(target_core, &nums);
                if rs.is_err() {
                    continue;
                }
                return rs;
            }
        }
    }
    Err(anyhow!("Unexpected error in reading cgroup directory."))
}

fn init_group(core: &str, nums: &Vec<&str>) -> Result<Box<[u8]>> {
    if !nums.contains(&core) {
        return Err(anyhow!("With no need for init group."));
    }
    let mut need_init: Vec<u8> = Vec::new();
    for i in nums {
        let Ok(i) = i.parse::<u8>() else { continue };
        need_init.push(i);
    }
    Ok(need_init.into_boxed_slice())
}
