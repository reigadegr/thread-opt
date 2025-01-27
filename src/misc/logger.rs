//From shadow3aaa fas-rs
use anyhow::Result;
use chrono::{DateTime, FixedOffset, Utc};
use flexi_logger::{DeferredNow, LogSpecification, Logger, Record};
// use log::info;
use std::io::{self, prelude::*};
use tracing::info;
use tracing_subscriber::fmt;
use tracing_subscriber::EnvFilter;

pub fn init_log() {
    let env_filter = if cfg!(debug_assertions) {
        EnvFilter::new("debug")
    } else {
        EnvFilter::new("info")
    };

    tracing_subscriber::fmt().with_env_filter(env_filter).init();
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
