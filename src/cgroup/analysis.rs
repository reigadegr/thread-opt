use super::group_info::{get_background_group, get_top_group};
use crate::utils::node_reader::read_file;
use anyhow::{Result, anyhow};
use compact_str::CompactString;
use libc::{DT_DIR, closedir, opendir, readdir};
use likely_stable::unlikely;
use log::info;
use once_cell::sync::Lazy;
use std::ffi::CStr;
use std::ffi::CString;
use stringzilla::sz;
extern crate alloc;
use alloc::{boxed::Box, vec::Vec};

pub static TOP_GROUP: Lazy<Box<[u8]>> = Lazy::new(|| analysis_cgroup_new("7").unwrap());

pub static BACKEND_GROUP: Lazy<Box<[u8]>> = Lazy::new(|| analysis_cgroup_new("0").unwrap());

pub static MIDDLE_GROUP: Lazy<Box<[u8]>> = Lazy::new(|| {
    let mut all_core: Vec<u8> = [0, 1, 2, 3, 4, 5, 6, 7].to_vec();
    let background_values = get_background_group();
    let top_values = get_top_group();

    for &value in background_values.iter().chain(top_values.iter()) {
        all_core.retain(|&x| x != value);
    }

    if all_core.is_empty() {
        info!("MIDDLE_GROUP initializing with BACKEND_GROUP.");
        background_values.into()
    } else {
        // 否则，使用处理后的 all_core 初始化 MIDDLE_GROUP
        all_core.into_boxed_slice()
    }
});

pub fn analysis_cgroup_new(target_core: &str) -> Result<Box<[u8]>> {
    let cgroup = "/sys/devices/system/cpu/cpufreq";
    #[cfg(debug_assertions)]
    let start = std::time::Instant::now();
    let cgroup = CString::new(cgroup)?;
    let dir = unsafe { opendir(cgroup.as_ptr()) };

    if unlikely(dir.is_null()) {
        return Err(anyhow!("Cannot read task_dir."));
    }

    let mut entries = Vec::new();
    unsafe {
        let dir_ptr = dir;
        loop {
            let entry = readdir(dir_ptr);
            if unlikely(entry.is_null()) {
                break;
            }

            // 获取目录项的名称
            let d_name_ptr = (*entry).d_name.as_ptr();
            let c_str = CStr::from_ptr(d_name_ptr);
            let entry_name = c_str.to_str()?;
            let file_type = (*entry).d_type;
            if file_type == DT_DIR && !entry_name.starts_with('.') {
                entries.push(entry_name.to_owned());
            }
        }
        // 关闭目录
        closedir(dir_ptr);
    }

    for entry in entries {
        let entry = format!("/sys/devices/system/cpu/cpufreq/{entry}");
        let entry = CString::new(entry)?;
        let core_dir_ptr = unsafe { opendir(entry.as_ptr()) };

        if unlikely(core_dir_ptr.is_null()) {
            return Err(anyhow!("Cannot read cgroup dir."));
        }

        unsafe {
            let dir_ptr = core_dir_ptr;

            loop {
                let entry_ptr = readdir(dir_ptr);
                if unlikely(entry_ptr.is_null()) {
                    break;
                }
                let d_name_ptr = (*entry_ptr).d_name.as_ptr();
                // 这里，最大为related_cpus的长度，12
                let bytes = std::slice::from_raw_parts(d_name_ptr, 12);
                if sz::find(bytes, b"related_cpus").is_some() {
                    let bytes = std::str::from_utf8(bytes).unwrap();
                    let bytes = format!("{}/{bytes}", entry.to_str().unwrap());
                    let content = read_file(&bytes).unwrap_or_else(|_| CompactString::new("8"));
                    // 解析文件内容
                    let nums: Vec<&str> = content.split_whitespace().collect();
                    let rs = init_group(target_core, &nums);
                    if rs.is_err() {
                        continue;
                    }
                    #[cfg(debug_assertions)]
                    {
                        let end = start.elapsed();
                        log::debug!("读目录时间: {:?}", end);
                    }
                    closedir(dir_ptr);
                    return rs;
                }
            }
            closedir(core_dir_ptr);
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
