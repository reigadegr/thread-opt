pub mod format_profile;
use std::{collections::HashSet, env, fs, sync::Arc};

use anyhow::Result;
use arc_swap::{ArcSwap, Guard};
use compact_str::CompactString;
use format_profile::format_toml;
use log::{error, info};
use serde::{
    Deserialize,
    de::{Error, SeqAccess, Visitor},
};

use crate::policy::usage_top1::common::CmdType as Top1Enum;

const MAX_COMM_SIZE: usize = 16;
const DEFAULT_PROFILE: &str = "/data/adb/modules/thread_opt/thread_opt.toml";

pub type ByteArray = heapless::Vec<u8, MAX_COMM_SIZE>;

pub struct AtomicConfig {
    inner: ArcSwap<Config>,
    profile: String,
}

impl AtomicConfig {
    pub fn init() -> Result<Self> {
        let profile = profile_path();
        let raw_content = fs::read_to_string(&profile)?;
        let formatted_content = format_toml(&raw_content);
        let _ = fs::write(&profile, formatted_content);

        let config = toml::from_str(&raw_content)?;

        Ok(Self {
            inner: ArcSwap::from(Arc::new(config)),
            profile,
        })
    }

    pub fn get(&self) -> Guard<Arc<Config>> {
        self.inner.load()
    }

    pub fn reload(&self) {
        let raw_content = match fs::read_to_string(&self.profile) {
            Ok(raw_content) => raw_content,
            Err(e) => {
                error!(
                    "Failed to reload config: cannot read {}: {e}.",
                    self.profile
                );
                return;
            }
        };

        let new_config = match toml::from_str(&raw_content) {
            Ok(new_config) => new_config,
            Err(e) => {
                error!(
                    "Failed to reload config: cannot parse {}: {e}.",
                    self.profile
                );
                return;
            }
        };

        self.inner.store(Arc::new(new_config));
        info!("Config profile reloaded successfully.");
    }
}

pub fn profile_path() -> String {
    match env::args().nth(1) {
        Some(profile) => profile,
        None => DEFAULT_PROFILE.to_string(),
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
    #[serde(default, deserialize_with = "deserialize_vec")]
    pub all: Box<[ByteArray]>,
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

struct ByteArrayListVisitor;

impl<'de> Visitor<'de> for ByteArrayListVisitor {
    type Value = Vec<ByteArray>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("a sequence of thread name prefixes")
    }

    fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut prefixes = if let Some(capacity) = sequence.size_hint() {
            Vec::with_capacity(capacity)
        } else {
            Vec::new()
        };

        while let Some(prefix) = sequence.next_element::<CompactString>()? {
            prefixes.push(str_to_byte_array(&prefix).map_err(Error::custom)?);
        }

        Ok(prefixes)
    }
}

fn deserialize_byte_arrays<'de, D>(deserializer: D) -> Result<Vec<ByteArray>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserializer.deserialize_seq(ByteArrayListVisitor)
}

fn deserialize_vec<'de, D>(deserializer: D) -> Result<Box<[ByteArray]>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserialize_byte_arrays(deserializer).map(Vec::into_boxed_slice)
}

fn deserialize_optional<'de, D>(deserializer: D) -> Result<Option<ByteArray>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Ok(deserialize_byte_arrays(deserializer)?.into_iter().next())
}

fn deserialize_single<'de, D>(deserializer: D) -> Result<ByteArray, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserialize_optional(deserializer).map(|value| match value {
        Some(value) => value,
        None => ByteArray::default(),
    })
}

#[cfg(test)]
mod tests {
    use super::{MAX_COMM_SIZE, Policy, UsageTop2};

    #[test]
    fn policy_without_all_field_remains_compatible() {
        let policy: Policy = match toml::from_str(
            r"
                top = []
                dualo = []
                only7 = []
                middle = []
                mono = []
                background = []
            ",
        ) {
            Ok(policy) => policy,
            Err(error) => panic!("test policy should deserialize: {error}"),
        };

        assert!(policy.all.is_empty());
    }

    #[test]
    fn thread_name_prefixes_are_truncated_and_optional_values_use_the_first_item() {
        let config: UsageTop2 = match toml::from_str(
            r#"
                packages = ["com.example.game"]
                max_comm = ["abcdefghijklmnop-extra"]
                second_comm = ["first", "ignored"]
            "#,
        ) {
            Ok(config) => config,
            Err(error) => panic!("usage_top2 should deserialize: {error}"),
        };

        assert_eq!(config.max_comm.as_slice(), b"abcdefghijklmnop");
        assert_eq!(config.max_comm.len(), MAX_COMM_SIZE);
        assert_eq!(config.second_comm.as_deref(), Some(&b"first"[..]));
    }
}
