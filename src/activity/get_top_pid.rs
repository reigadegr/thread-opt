use dumpsys_rs::BoundDumpsys;
use likely_stable::LikelyOption;
use log::{error, info};
use memchr::{memchr, memmem, memrchr};

use crate::utils::sleep::sleep_millis;

/// 匹配 `dumpsys activity lru` 中的前台 TOP 行，抓取 PID：
/// `#59: fg       TOP  LCMNFUATI 28546:bin.mt.plus/u0a328 act:activities|recents`

#[derive(Default)]
pub struct TopPidInfo {
    pid: i32,
}

fn extract_pid(line: &[u8]) -> Option<i32> {
    let top = memmem::find(line, b" TOP")?;
    let colon = top + memchr(b':', &line[top..])?;
    let space = memrchr(b' ', &line[..colon])?;
    let digits = &line[space + 1..colon];

    atoi_simd::parse::<i32, false, false>(digits).ok()
}

impl TopPidInfo {
    pub fn new(dump: &[u8]) -> Self {
        let top_flag = memmem::Finder::new(b" TOP");
        let pid = dump
            .split(|b| *b == b'\n')
            .find(|line| top_flag.find(line).is_some())
            .and_then_likely(extract_pid)
            .unwrap_or(0);
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
