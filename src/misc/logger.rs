// From shadow3aaa fas-rs
use tklog::{LEVEL, LOG, info};

pub fn init_log() {
    LOG.set_console(true)
        .set_level(LEVEL::Trace)
        .set_formatter("[{time}] {level}: {message}\n")
        .uselog();
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
