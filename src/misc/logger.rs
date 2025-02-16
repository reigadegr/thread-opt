// From shadow3aaa fas-rs
use anyhow::Result;
use flexi_logger::{DeferredNow, LogSpecification, Logger, Record};
use log::info;
use std::io::Write;

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
) -> Result<(), std::io::Error> {
    let time = now.format("%Y-%m-%d %H:%M:%S");
    write!(write, "[{time}] {}: {}", record.level(), record.args())
}

pub fn log_metainfo() {
    info!(
        "thread-opt v{} {}, llvm-{}, rustc-{}, on {},{},{}",
        env!("CARGO_PKG_VERSION"),
        build_type(),
        env!("VERGEN_RUSTC_LLVM_VERSION"),
        env!("VERGEN_RUSTC_SEMVER"),
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
