pub mod format_profile;
use crate::{
    policy::usage_top1::common::CmdType as Top1Enum,
    utils::node_reader::{read_file, write_to_byte},
};
use compact_str::CompactString;
use serde::Deserialize;
extern crate alloc;
use alloc::{boxed::Box, vec::Vec};
use anyhow::Result;
use format_profile::format_toml;
use hashbrown::HashSet;
use std::sync::LazyLock;

pub type ByteArray = heapless::Vec<u8, 16>;
pub static PROFILE: LazyLock<Config> = LazyLock::new(|| {
    let profile_path = b"/data/adb/modules/thread_opt/thread_opt.toml\0";
    let profile = read_file::<65536>(profile_path).unwrap();
    let format_rs = format_toml(&profile);
    let profile: Config = toml::from_str(&profile).unwrap();
    write_to_byte(profile_path, format_rs.as_bytes()).unwrap();
    profile
});

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
    #[serde(deserialize_with = "deserialize_byte_array_one")]
    pub max_comm: ByteArray,
    pub max_comm_core: Top1Enum,
    pub policy: Policy,
}

#[derive(Deserialize, Eq, Hash, PartialEq)]
pub struct UsageTop2 {
    pub packages: Box<[CompactString]>,
    #[serde(deserialize_with = "deserialize_byte_array_one")]
    pub max_comm: ByteArray,
    #[serde(default, deserialize_with = "deserialize_byte_array_one_op")]
    pub second_comm: Option<ByteArray>,
}

#[derive(Deserialize, Eq, Hash, PartialEq)]
pub struct Policy {
    #[serde(deserialize_with = "deserialize_byte_array")]
    pub top: Box<[ByteArray]>,
    #[serde(deserialize_with = "deserialize_byte_array")]
    pub dualo: Box<[ByteArray]>,
    #[serde(deserialize_with = "deserialize_byte_array")]
    pub only7: Box<[ByteArray]>,
    #[serde(deserialize_with = "deserialize_byte_array")]
    pub middle: Box<[ByteArray]>,
    #[serde(deserialize_with = "deserialize_byte_array")]
    pub mono: Box<[ByteArray]>,
    #[serde(deserialize_with = "deserialize_byte_array")]
    pub background: Box<[ByteArray]>,
}

fn deserialize_byte_array<'de, D>(deserializer: D) -> Result<Box<[ByteArray]>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let strings: Box<[CompactString]> = Vec::deserialize(deserializer)?.into();
    let mut result = Vec::new();
    for s in strings {
        let mut bytes = s.as_bytes();
        if bytes.len() > 16 {
            bytes = &bytes[..16];
        }
        let vec = heapless::Vec::from_slice(bytes)
            .map_err(|()| serde::de::Error::custom("String exceeds capacity"))?;
        result.push(vec);
    }
    Ok(result.into())
}

fn deserialize_byte_array_one<'de, D>(deserializer: D) -> Result<ByteArray, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let strings: Box<[CompactString]> = Vec::deserialize(deserializer)?.into();
    if let Some(s) = strings.into_iter().next() {
        let mut bytes = s.as_bytes();
        if bytes.len() > 16 {
            bytes = &bytes[..16];
        }
        let vec = heapless::Vec::from_slice(bytes)
            .map_err(|()| serde::de::Error::custom("String exceeds capacity"))?;
        return Ok(vec);
    }
    Ok(heapless::Vec::new())
}

fn deserialize_byte_array_one_op<'de, D>(deserializer: D) -> Result<Option<ByteArray>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let strings: Box<[CompactString]> = Vec::deserialize(deserializer)?.into();
    if let Some(s) = strings.into_iter().next() {
        let mut bytes = s.as_bytes();
        if bytes.len() > 16 {
            bytes = &bytes[..16];
        }
        let vec = heapless::Vec::from_slice(bytes)
            .map_err(|()| serde::de::Error::custom("String exceeds capacity"))?;
        return Ok(Some(vec));
    }
    Ok(None)
}
