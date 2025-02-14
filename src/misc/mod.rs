pub mod logger;
use crate::cgroup::group_info::print_group_core;
use anyhow::Result;
use likely_stable::unlikely;
use log::info;
use logger::{init_log, log_metainfo};

pub fn init_misc() {
    working_in_background();
    let rs = set_main_thread_name("AffinitySetter");
    if unlikely(rs.is_err()) {
        info!("Cannot rename the main thread name.");
    }
    let _ = init_log();
    log_metainfo();
    print_group_core();
    print_misc();
}

fn working_in_background() {
    let self_pid = std::process::id();
    let _ = std::fs::write("/dev/cpuset/background/tasks", self_pid.to_string());
}

fn set_main_thread_name(name: &str) -> Result<()> {
    let truncated_name = if unlikely(name.len() > 15) {
        &name[..15]
    } else {
        name
    };

    let thread_name = std::ffi::CString::new(truncated_name)?;
    unsafe {
        libc::pthread_setname_np(libc::pthread_self(), thread_name.as_ptr());
    }
    Ok(())
}

fn print_misc() {
    info!("免费软件，禁止商用");
    info!("Free software, not for commercial use.");
    info!("祝各位大佬们情人节快乐，在这个充满爱意的日子 里，愿单身的您能邂逅心动，开启浪漫篇章；有伴的您与爱人情比金坚，甜蜜满溢。生活不止忙碌奔波，还有此刻的温柔缱绻。愿大家都能被爱包围，幸福长存，每一天都如情人节般美好！😘🌹");
    info!("Happy Valentine's Day to all the big shots out there! May those who are single encounter someone who makes their heart skip a beat and start a romantic journey. And for those who are already in a relationship, may you and your loved one have an unbreakable bond and be filled with sweetness. Life is not just about the hustle and bustle; there are also moments of tenderness and affection like this one. I hope everyone can be surrounded by love, enjoy lasting happiness, and have every day be as wonderful as Valentine's Day！😘🌹");
}
