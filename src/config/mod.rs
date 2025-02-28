use crate::utils::node_reader::read_file;
use compact_str::{CompactString, ToCompactString};
use serde::Deserialize;
extern crate alloc;
use alloc::{boxed::Box, vec::Vec};
use once_cell::sync::Lazy;

pub static PROFILE: Lazy<Config> = Lazy::new(|| {
    let config = read_file::<65536>(b"./thread_opt.toml\0").unwrap();
    log::info!("{config:?}");

    let config: Config = toml::from_str(&config).unwrap();
    config
});

#[derive(Deserialize)]
pub struct Config {
    mode: i32,
    pub unity: NameMatch,
}

#[derive(Deserialize)]
pub struct NameMatch {
    pub packages: Vec<CompactString>,
}

pub fn get_packages(vec: Vec<CompactString>) -> &'static [&'static str] {
    let mut leaked_strings: Vec<&'static str> = Vec::new();

    for value in vec {
        let s = value.as_str().to_compact_string();
        let leaked_mut = Box::leak(Box::new(s));
        leaked_strings.push(&*leaked_mut);
    }
    #[cfg(debug_assertions)]
    log::debug!("{leaked_strings:?}");

    Box::leak(Box::new(leaked_strings))
}
