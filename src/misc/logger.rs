//From shadow3aaa fas-rs
use anyhow::Result;
use chrono::{DateTime, FixedOffset, Utc};
use flexi_logger::{DeferredNow, LogSpecification, Logger, Record};
use log::info;
use std::io::{self, prelude::*};
pub fn init_log() -> Result<()> {
    let logger_spec = if cfg!(debug_assertions) {
        LogSpecification::debug()
    } else {
        LogSpecification::info()
    };

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

pub fn log_metainfo() {
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
        "Debug build"
    } else {
        "Release build"
    }
}

fn utc_plus_8_time() -> String {
    let build_timestamp = env!("VERGEN_BUILD_TIMESTAMP");
    let utc_time: DateTime<Utc> = match build_timestamp.parse() {
        Ok(utc_time) => utc_time,
        Err(_) => return build_timestamp.to_string(),
    };
    let Some(utc_plus_8) = FixedOffset::east_opt(8 * 3600) else {
        return build_timestamp.to_string();
    };
    let utc_plus_8_time = utc_time.with_timezone(&utc_plus_8);
    utc_plus_8_time.to_rfc3339()
}
