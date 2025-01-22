use super::activity::get_tid_info::read_file;
use anyhow::Context;
use libc::pid_t;
use log::info;
use std::fs;
pub mod dir_ctrl;
use super::fs_utils::dir_ctrl::{
    create_parent_dir, create_sub_work_space, get_background_dir, get_middle_dir, get_top_dir,
    middle_dir_ctrl,
};
pub mod node_writer;

pub fn init_working_directory() -> anyhow::Result<()> {
    let _ = analysis_cgroup();
    let _ = middle_dir_ctrl();
    let rs1 = get_top_dir()?;
    let rs2 = get_background_dir()?;
    let rs3 = get_middle_dir()?;
    info!("\ntop: {}\nbackground: {}\nmiddle: {}", rs1, rs2, rs3);
    Ok(())
}

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
                let content = match read_file(&path) {
                    Ok(number) => number,
                    Err(_) => "0".to_string(),
                };

                // 解析文件内容，提取第一个和最后一个数字
                let nums: Vec<&str> = content.split_whitespace().collect();
                if let (Some(first), Some(last)) = (nums.first(), nums.last()) {
                    let first_num = first.parse::<pid_t>().with_context(|| {
                        format!("Failed to parse first number in file: {}", path.display())
                    })?;
                    let last_num = last.parse::<pid_t>().with_context(|| {
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
