pub mod format_profile;
use crate::policy::usage_top1::common::CmdType as Top1Enum;
use anyhow::Result;
use arc_swap::{ArcSwap, Guard};
use compact_str::CompactString;
use format_profile::format_toml;
use log::{error, info};
use serde::{Deserialize, de::Error};
use std::{collections::HashSet, env, fs, sync::Arc};

const MAX_COMM_SIZE: usize = 16;

pub type ByteArray = heapless::Vec<u8, MAX_COMM_SIZE>;

pub struct AtomicConfig {
    inner: ArcSwap<Config>,
    profile: String,
}

impl AtomicConfig {
    pub fn init() -> Self {
        let profile = env::args()
            .nth(1)
            .unwrap_or_else(|| "/data/adb/modules/thread_opt/thread_opt.toml".to_string());
        let raw_content = fs::read_to_string(&profile).expect("Failed to read thread_opt.toml");
        let formatted_content = format_toml(&raw_content);
        let _ = fs::write(&profile, formatted_content);

        let config = toml::from_str(&raw_content)
            .expect("Failed to parse thread_opt.toml. Please check syntax.");

        Self {
            inner: ArcSwap::from(Arc::new(config)),
            profile,
        }
    }

    pub fn get(&self) -> Guard<Arc<Config>> {
        self.inner.load()
    }

    pub fn reload(&self) {
        match std::panic::catch_unwind(|| {
            let raw_content = fs::read_to_string(&self.profile)
                .expect("Failed to read thread_opt.toml during reload");

            let new_config = toml::from_str(&raw_content)
                .expect("Failed to parse thread_opt.toml during reload");

            Arc::new(new_config)
        }) {
            Ok(new_config_arc) => {
                self.inner.store(new_config_arc);
                info!("Config profile reloaded successfully.");
            }
            Err(_) => {
                error!("Failed to reload config: Parsing panicked.");
            }
        }
    }
}

#[derive(Deserialize)]
pub struct Config {
    pub comm_match: HashSet<NameMatch>,
    pub usage_top1: HashSet<UsageTop1>,
    pub usage_top2: HashSet<UsageTop2>,
}

#[derive(Deserialize, Eq, Hash, PartialEq)]
pub struct NameMatch {
    pub packages: Box<[CompactString]>,
    pub policy: Policy,
}

#[derive(Deserialize, Eq, Hash, PartialEq)]
pub struct UsageTop1 {
    pub packages: Box<[CompactString]>,
    #[serde(deserialize_with = "deserialize_single")]
    pub max_comm: ByteArray,
    pub max_comm_core: Top1Enum,
    pub policy: Policy,
}

#[derive(Deserialize, Eq, Hash, PartialEq)]
pub struct UsageTop2 {
    pub packages: Box<[CompactString]>,
    #[serde(deserialize_with = "deserialize_single")]
    pub max_comm: ByteArray,
    #[serde(default, deserialize_with = "deserialize_optional")]
    pub second_comm: Option<ByteArray>,
}

#[derive(Deserialize, Eq, Hash, PartialEq)]
pub struct Policy {
    #[serde(deserialize_with = "deserialize_vec")]
    pub top: Box<[ByteArray]>,
    #[serde(deserialize_with = "deserialize_vec")]
    pub dualo: Box<[ByteArray]>,
    #[serde(deserialize_with = "deserialize_vec")]
    pub only7: Box<[ByteArray]>,
    #[serde(deserialize_with = "deserialize_vec")]
    pub middle: Box<[ByteArray]>,
    #[serde(deserialize_with = "deserialize_vec")]
    pub mono: Box<[ByteArray]>,
    #[serde(deserialize_with = "deserialize_vec")]
    pub background: Box<[ByteArray]>,
}

fn str_to_byte_array(s: &str) -> Result<ByteArray, String> {
    let bytes = s.as_bytes();
    let slice = if bytes.len() > MAX_COMM_SIZE {
        &bytes[..MAX_COMM_SIZE]
    } else {
        bytes
    };

    heapless::Vec::from_slice(slice).map_err(|e| e.to_string())
}

fn deserialize_vec<'de, D>(deserializer: D) -> Result<Box<[ByteArray]>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let strings: Box<[CompactString]> = Vec::deserialize(deserializer)?.into();
    strings
        .iter()
        .map(|s| str_to_byte_array(s).map_err(Error::custom))
        .collect::<Result<Vec<_>, _>>()
        .map(Vec::into_boxed_slice)
}

fn deserialize_optional<'de, D>(deserializer: D) -> Result<Option<ByteArray>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let strings: Box<[CompactString]> = Vec::deserialize(deserializer)?.into();
    match strings.first() {
        Some(s) => Ok(Some(str_to_byte_array(s).map_err(Error::custom)?)),
        None => Ok(None),
    }
}

fn deserialize_single<'de, D>(deserializer: D) -> Result<ByteArray, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserialize_optional(deserializer).map(Option::unwrap_or_default)
}
