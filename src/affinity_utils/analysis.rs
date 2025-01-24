use crate::fs_utils::node_reader::read_file;
use anyhow::{anyhow, Context, Result};
use log::info;
use once_cell::sync::OnceCell;
use std::fs;

pub static TOP_GROUP: OnceCell<Box<[u8]>> = OnceCell::new();

pub static MIDDLE_GROUP: OnceCell<Box<[u8]>> = OnceCell::new();

pub static BACKEND_GROUP: OnceCell<Box<[u8]>> = OnceCell::new();

pub fn analysis_cgroup_new() -> Result<()> {
    let cgroup = "/sys/devices/system/cpu/cpufreq";
    let entries = fs::read_dir(cgroup).context("Failed to read directory")?;
    for entry in entries {
        let entry =
            entry.with_context(|| format!("Failed to read entry in directory: {}", cgroup))?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let core_dir = fs::read_dir(path).context("Failed to read directory")?;

        for file in core_dir {
            let file =
                file.with_context(|| format!("Failed to read entry in directory: {}", cgroup))?;
            let path = file.path();

            // 检查文件名是否包含 "related_cpus"
            if path
                .file_name()
                .and_then(|f| f.to_str())
                .is_some_and(|f| f.contains("related_cpus"))
            {
                let content = match read_file(&path) {
                    Ok(number) => number,
                    Err(_) => "0".to_string(),
                };

                // 解析文件内容
                let nums: Vec<&str> = content.split_whitespace().collect();

                let _ = init_group("7", &nums, &TOP_GROUP);
                let _ = init_group("0", &nums, &BACKEND_GROUP);
            }
        }
    }

    let mut all_core: Vec<u8> = [0, 1, 2, 3, 4, 5, 6, 7].to_vec();
    let backend_values = BACKEND_GROUP.get().unwrap();
    let top_values = TOP_GROUP.get().unwrap();

    for &value in backend_values.iter().chain(top_values.iter()) {
        all_core.retain(|&x| x != value);
    }

    if all_core.is_empty() {
        info!("MIDDLE_GROUP initializing with BACKEND_GROUP.");
        MIDDLE_GROUP.set(backend_values.clone()).unwrap();
    } else {
        // 否则，使用处理后的 all_core 初始化 MIDDLE_GROUP
        MIDDLE_GROUP.set(all_core.into_boxed_slice()).unwrap();
    }

    Ok(())
}

fn init_group(core: &str, nums: &Vec<&str>, target_group: &OnceCell<Box<[u8]>>) -> Result<()> {
    if !nums.contains(&core) {
        return Err(anyhow!("With no need for init group."));
    }
    let mut need_init: Vec<u8> = Vec::new();
    for i in nums {
        let i = i.parse::<u8>()?;
        need_init.push(i);
    }
    target_group.get_or_init(|| need_init.into_boxed_slice());
    Ok(())
}
