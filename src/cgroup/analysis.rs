use super::group_info::{get_background_group, get_top_group};
use crate::utils::node_reader::read_file;
use anyhow::{Result, anyhow};
use compact_str::CompactString;
use likely_stable::unlikely;
use log::info;
use once_cell::sync::Lazy;
extern crate alloc;
use alloc::vec::Vec;

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
    let entries = std::fs::read_dir(cgroup)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if unlikely(!path.is_dir()) {
            continue;
        }

        let core_dir = std::fs::read_dir(path)?;

        for file in core_dir {
            let file = file?;
            let path = file.path();

            // 检查文件名是否包含 "related_cpus"
            if path
                .file_name()
                .and_then(|f| f.to_str())
                .is_some_and(|f| f.contains("related_cpus"))
            {
                let content = read_file(&path).unwrap_or_else(|_| CompactString::new("8"));

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
        let i = i.parse::<u8>().unwrap();
        need_init.push(i);
    }
    Ok(need_init.into_boxed_slice())
}
