use super::{
    name_match::policies::{policy_sky, policy_ue, policy_unity},
    usage_top1::policies::{
        policy_cocos, policy_ru, policy_top1, policy_ue5, policy_unity_t1, policy_unity_t1_u2,
        policy_wzm,
    },
    usage_top2::{policy_party, policy_top2, policy_ue_t2, policy_unity_t2},
};
use crate::activity::ActivityUtils;
use libc::pid_t;
use once_cell::sync::Lazy;
use serde::Deserialize;
extern crate alloc;
use alloc::{boxed::Box, string::ToString, vec::Vec};

// 配置文件解析结构
#[derive(Debug, Deserialize)]
struct RawConfig {
    unity: Vec<toml::Value>,
    ue: Vec<toml::Value>,
    ue_t1: Vec<toml::Value>,
    ue5_t1: Vec<toml::Value>,
    wzm_t1: Vec<toml::Value>,
    ru_t1: Vec<toml::Value>,
    cocos_t1: Vec<toml::Value>,
    unity_t1: Vec<toml::Value>,
    unity_t1_u2: Vec<toml::Value>,
    usage_t2: Vec<toml::Value>,
    unity_t2: Vec<toml::Value>,
    party_t2: Vec<toml::Value>,
    ue_t2: Vec<toml::Value>,
    sky_t2: Vec<toml::Value>,
}

// 静态配置转换器
fn convert_to_static(vec: Vec<toml::Value>) -> &'static [&'static str] {
    let leaked_strings: Vec<&'static str> = vec
        .into_iter()
        .map(|value| {
            // 提取字符串并转换为 String
            let s = value.as_str().expect("Value is not a string").to_string();
            // 将 String 转为 Box<str>，泄漏后得到 &'static mut str
            let leaked_mut = Box::leak(s.into_boxed_str());
            // 将 &mut str 转为 &str
            &*leaked_mut
        })
        .collect();
    log::info!("{leaked_strings:?}");

    // 将 Vec 转为 Box<[&str]> 并泄漏为静态切片
    Box::leak(leaked_strings.into_boxed_slice())
}

// 懒加载配置
static CONFIG: Lazy<RawConfig> = Lazy::new(|| {
    let config_str = include_str!("../../game_config.toml");
    toml::from_str(config_str).expect("Failed to parse config")
});

// 静态策略配置
static STRATEGY_CONFIG: Lazy<StrategyConfig> = Lazy::new(|| StrategyConfig {
    unity: convert_to_static(CONFIG.unity.clone()),
    ue: convert_to_static(CONFIG.ue.clone()),
    ue_t1: convert_to_static(CONFIG.ue_t1.clone()),
    ue5_t1: convert_to_static(CONFIG.ue5_t1.clone()),
    wzm_t1: convert_to_static(CONFIG.wzm_t1.clone()),
    ru_t1: convert_to_static(CONFIG.ru_t1.clone()),
    cocos_t1: convert_to_static(CONFIG.cocos_t1.clone()),
    unity_t1: convert_to_static(CONFIG.unity_t1.clone()),
    unity_t1_u2: convert_to_static(CONFIG.unity_t1_u2.clone()),
    usage_t2: convert_to_static(CONFIG.usage_t2.clone()),
    unity_t2: convert_to_static(CONFIG.unity_t2.clone()),
    party_t2: convert_to_static(CONFIG.party_t2.clone()),
    ue_t2: convert_to_static(CONFIG.ue_t2.clone()),
    sky_t2: convert_to_static(CONFIG.sky_t2.clone()),
});

struct StrategyConfig {
    unity: &'static [&'static str],
    ue: &'static [&'static str],
    ue_t1: &'static [&'static str],
    ue5_t1: &'static [&'static str],
    wzm_t1: &'static [&'static str],
    ru_t1: &'static [&'static str],
    cocos_t1: &'static [&'static str],
    unity_t1: &'static [&'static str],
    unity_t1_u2: &'static [&'static str],
    usage_t2: &'static [&'static str],
    unity_t2: &'static [&'static str],
    party_t2: &'static [&'static str],
    ue_t2: &'static [&'static str],
    sky_t2: &'static [&'static str],
}

pub struct StartArgs<'a> {
    pub activity_utils: &'a mut ActivityUtils,
    pub pid: pid_t,
}

type ConfigTuple = (&'static [&'static str], fn(&mut StartArgs));

pub static PACKAGE_CONFIGS: Lazy<[ConfigTuple; 14]> = Lazy::new(|| {
    [
        (STRATEGY_CONFIG.ue, policy_ue::start_task),
        (STRATEGY_CONFIG.unity, policy_unity::start_task),
        (STRATEGY_CONFIG.ue_t1, policy_top1::start_task),
        (STRATEGY_CONFIG.ue5_t1, policy_ue5::start_task),
        (STRATEGY_CONFIG.wzm_t1, policy_wzm::start_task),
        (STRATEGY_CONFIG.ru_t1, policy_ru::start_task),
        (STRATEGY_CONFIG.cocos_t1, policy_cocos::start_task),
        (STRATEGY_CONFIG.unity_t1, policy_unity_t1::start_task),
        (STRATEGY_CONFIG.unity_t1_u2, policy_unity_t1_u2::start_task),
        (STRATEGY_CONFIG.usage_t2, policy_top2::start_task),
        (STRATEGY_CONFIG.unity_t2, policy_unity_t2::start_task),
        (STRATEGY_CONFIG.party_t2, policy_party::start_task),
        (STRATEGY_CONFIG.ue_t2, policy_ue_t2::start_task),
        (STRATEGY_CONFIG.sky_t2, policy_sky::start_task),
    ]
});
