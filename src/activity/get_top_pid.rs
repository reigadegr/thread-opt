use dumpsys_rs::BoundDumpsys;
use log::{error, info};
use memchr::{memchr, memmem, memrchr};

use crate::utils::sleep::sleep_millis;

/// 匹配 `dumpsys activity lru` 中的前台 TOP 行，抓取 PID：
/// `#59: fg       TOP  LCMNFUATI 28546:bin.mt.plus/u0a328 act:activities|recents`

#[derive(Default)]
pub struct TopPidInfo {
    pid: i32,
}

/// 命中 `" TOP "` 后只回溯到所在行，取最后一个空格到 `:` 之间的数字。
/// 整块只做一次 `memmem` 子串搜索，命中行不会被重复扫描。
fn extract_pid(dump: &[u8]) -> Option<i32> {
    let hit = memmem::find(dump, b" TOP ")?;
    let rest = &dump[hit..];
    let eol = memchr(b'\n', rest).unwrap_or(rest.len());
    let line = &rest[..eol];
    let colon = memchr(b':', line)?;
    let space = memrchr(b' ', &line[..colon])?;
    atoi_simd::parse::<i32, false, false>(&line[space + 1..colon]).ok()
}

impl TopPidInfo {
    pub fn new(dump: &[u8]) -> Self {
        Self {
            pid: extract_pid(dump).unwrap_or(0),
        }
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
