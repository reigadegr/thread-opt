use dumpsys_rs::BoundDumpsys;
use log::{error, info};
use regex::Regex;
use std::sync::LazyLock;

use crate::utils::sleep::sleep_millis;

/// 匹配 `dumpsys activity lru` 中的前台 TOP 行，抓取 PID：
/// `#59: fg       TOP  LCMNFUATI 28546:bin.mt.plus/u0a328 act:activities|recents`
///
/// - `[ \t]*` 而非 `\s*`，避免 `(?m)` 下 `^` 跨行吞掉空行
/// - `\s+fg\s+TOP\b` 排除 `vis + 2 TOP` 之类非前台行
/// - 唯一捕获组 `(\d+)` 就是 PID；`[A-Za-z0-9_.]+/` 用非捕获写法，
///   仅作为锚点排除 `act:activities|recents` 这类假冒号
static FG_TOP_RE: LazyLock<Regex> = LazyLock::new(|| {
    let pattern = r"(?m)^[ \t]*#\d+:\s+fg\s+TOP\b.*?\s(\d+):[A-Za-z0-9_.]+/";
    match Regex::new(pattern) {
        Ok(re) => re,
        Err(e) => panic!("FG_TOP_RE 正则字面量非法: {e}"),
    }
});

#[derive(Default)]
pub struct TopPidInfo {
    pid: i32,
}

impl TopPidInfo {
    pub fn new(dump: &[u8]) -> Self {
        let text = String::from_utf8_lossy(dump);
        let pid = match FG_TOP_RE.captures(&text) {
            Some(caps) => match caps[1].parse::<i32>() {
                Ok(pid) => pid,
                Err(_) => i32::default(),
            },
            None => i32::default(),
        };
        Self { pid }
    }
}

pub struct TopAppUtils {
    dumper: BoundDumpsys,
}

impl TopAppUtils {
    fn init_dumper() -> BoundDumpsys {
        loop {
            match BoundDumpsys::new("activity") {
                Ok(dumper) => break dumper,
                Err(e) => {
                    error!("Failed to bind dumpsys activity service: {e}, retrying");
                    sleep_millis(1000);
                }
            }
        }
    }

    pub fn new() -> Self {
        let dumper = Self::init_dumper();
        Self { dumper }
    }

    pub fn get_top_pid(&self) -> i32 {
        self.set_top_pid().pid
    }

    pub fn set_top_pid(&self) -> TopPidInfo {
        sleep_millis(900);
        let dump = loop {
            match self.dumper.dump_to_byte::<1024>(&["lru"]) {
                Ok(dump) => break dump,
                Err(e) => {
                    info!("Failed to dump lru: {e}, retrying");
                    sleep_millis(200);
                }
            }
        };
        TopPidInfo::new(&dump)
    }
}
