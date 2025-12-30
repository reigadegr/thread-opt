pub mod format_profile;
use crate::{
    policy::usage_top1::common::CmdType as Top1Enum,
    utils::node_reader::{read_file, write_to_byte},
};
use anyhow::Result;
use compact_str::CompactString;
use format_profile::format_toml;
use serde::Deserialize;
use std::{collections::HashSet, sync::LazyLock};

const MAX_COMM_SIZE: usize = 16;
const CONFIG_PATH: &[u8] = b"/data/adb/modules/thread_opt/thread_opt.toml\0";

pub type ByteArray = heapless::Vec<u8, MAX_COMM_SIZE>;

pub static PROFILE: LazyLock<Config> = LazyLock::new(load_profile);

fn load_profile() -> Config {
    let raw_content = read_file::<65536>(CONFIG_PATH).expect("Failed to read thread_opt.toml");

    let formatted_content = format_toml(&raw_content);
    write_to_byte(CONFIG_PATH, formatted_content.as_bytes())
        .expect("Failed to write formatted config back");

    toml::from_str(&raw_content).expect("Failed to parse thread_opt.toml. Please check syntax.")
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
        .map(|s| str_to_byte_array(s).map_err(serde::de::Error::custom))
        .collect::<Result<Vec<_>, _>>()
        .map(std::vec::Vec::into_boxed_slice)
}

fn deserialize_optional<'de, D>(deserializer: D) -> Result<Option<ByteArray>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let strings: Box<[CompactString]> = Vec::deserialize(deserializer)?.into();
    match strings.first() {
        Some(s) => Ok(Some(
            str_to_byte_array(s).map_err(serde::de::Error::custom)?,
        )),
        None => Ok(None),
    }
}

fn deserialize_single<'de, D>(deserializer: D) -> Result<ByteArray, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserialize_optional(deserializer).map(std::option::Option::unwrap_or_default)
}
