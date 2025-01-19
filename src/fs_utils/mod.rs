use crate::activity::get_tid_info::read_file;
use anyhow::Context;
use log::info;
use std::fs;
use std::path::Path;
pub mod dir_ctrl;
use crate::fs_utils::dir_ctrl::create_parent_dir;
use crate::fs_utils::dir_ctrl::create_sub_work_space;
pub fn analysis_cgroup() -> anyhow::Result<()> {
    create_parent_dir();
    let cgroup = "/sys/devices/system/cpu/cpufreq";
    let entries = fs::read_dir(cgroup).context("Failed to read directory")?;
    for entry in entries {
        let entry =
            entry.with_context(|| format!("Failed to read entry in directory: {}", cgroup))?;
        let path = entry.path();

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
                // info!("-{:?}-", path);
                // 读取文件内容
                let content = match read_file(&path) {
                    Ok(number) => number,
                    Err(_) => "0".to_string(),
                };

                // 解析文件内容，提取第一个和最后一个数字
                let nums: Vec<&str> = content.split_whitespace().collect();
                if let (Some(first), Some(last)) = (nums.first(), nums.last()) {
                    let first_num = first.parse::<i32>().with_context(|| {
                        format!("Failed to parse first number in file: {}", path.display())
                    })?;
                    let last_num = last.parse::<i32>().with_context(|| {
                        format!("Failed to parse last number in file: {}", path.display())
                    })?;

                    // 生成所需字符串
                    if first_num != last_num {
                        let result = format!("{}-{}", first_num, last_num);
                        // info!("{}: {}", path.display(), result);
                        create_sub_work_space(&result);
                    } else {
                        let result = format!("{}", first_num);
                        // info!("{}: {}", path.display(), result);
                        create_sub_work_space(&result);
                    }
                }
            }
        }
    }
    Ok(())
}
