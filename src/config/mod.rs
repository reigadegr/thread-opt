use crate::utils::node_reader::read_file;
use compact_str::CompactString;
use serde::Deserialize;
extern crate alloc;
use alloc::{sync::Arc, vec::Vec};
use once_cell::sync::{Lazy, OnceCell};

pub static PROFILE: Lazy<Config> = Lazy::new(|| {
    let config = read_file::<65536>(b"./thread_opt.toml\0").unwrap();
    log::info!("{config:?}");

    let config: Config = toml::from_str(&config).unwrap();
    config
});

#[derive(Deserialize)]
pub struct Config {
    pub unity: NameMatch,
    pub usage1: UsageTop1,
}

#[derive(Deserialize)]
pub struct NameMatch {
    pub packages: Vec<CompactString>,
}

#[derive(Deserialize)]
pub struct UsageTop1 {
    pub packages: Vec<CompactString>,
}

pub fn get_packages(vec: &[CompactString]) -> &'static [&'static str] {
    static CACHE: OnceCell<Arc<[&'static str]>> = OnceCell::new();

    // 获取或初始化缓存
    let cached = CACHE.get_or_init(|| {
        // 将 CompactString 转换为 &'static str
        let static_slices: Vec<&'static str> = vec
            .iter()
            .map(|cs| unsafe {
                // 安全条件：确保 CompactString 生命周期足够长
                core::mem::transmute::<&str, &'static str>(cs.as_str())
            })
            .collect();
        // 通过 Arc 共享内存
        Arc::from(static_slices.into_boxed_slice())
    });
    &cached[..]
}
