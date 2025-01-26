//From shadow3aaa fas-rs
use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use flexi_logger::{DeferredNow, LogSpecification, Logger, Record};
use log::info;
use std::{
    fs,
    io::{self, prelude::*},
    process,
};

fn init_log() -> Result<()> {
    unsafe {
        std::env::set_var("RUST_LOG", "info");
    }
    let logger_spec = LogSpecification::info();
    Logger::with(logger_spec)
        .log_to_stdout()
        .format(log_format)
        .start()?;
    Ok(())
}

fn log_format(
    write: &mut dyn Write,
    now: &mut DeferredNow,
    record: &Record<'_>,
) -> Result<(), io::Error> {
    let time = now.format("%Y-%m-%d %H:%M:%S");
    write!(write, "[{time}] {}: {}", record.level(), record.args())
}

fn log_metainfo() {
    info!(
        "thread-opt v{} {}, llvm-{}, rustc-{}, build by {} at {} on {},{},{}",
        env!("CARGO_PKG_VERSION"),
        build_type(),
        env!("VERGEN_RUSTC_LLVM_VERSION"),
        env!("VERGEN_RUSTC_SEMVER"),
        env!("VERGEN_SYSINFO_USER"),
        utc_plus_8_time(),
        env!("VERGEN_SYSINFO_NAME"),
        env!("VERGEN_SYSINFO_OS_VERSION"),
        env!("VERGEN_RUSTC_HOST_TRIPLE")
    );
}

const fn build_type() -> &'static str {
    if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    }
}

fn utc_plus_8_time() -> String {
    let build_timestamp = env!("VERGEN_BUILD_TIMESTAMP");
    let utc_time: DateTime<Utc> = match build_timestamp.parse() {
        Ok(time) => time,
        Err(_) => return build_timestamp.to_string(),
    };
    let utc_plus_8_time = utc_time + Duration::hours(8);
    let mut utc_plus_8_time = utc_plus_8_time.to_string();
    if let Some(pos) = utc_plus_8_time.find("UTC") {
        utc_plus_8_time.replace_range(pos..pos + 3, "UTC+8");
    }
    utc_plus_8_time
}

pub fn init_misc() {
    let _ = init_log();
    log_metainfo();
    let self_pid = process::id();
    let _ = fs::write("/dev/cpuset/background/tasks", self_pid.to_string());
}
