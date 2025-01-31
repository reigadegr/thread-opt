use super::{
    name_match::{policy_ue, policy_unity},
    usage_top2::policy_unname,
};
use crate::Controller;
use compact_str::CompactString;
use hashbrown::HashMap;
use libc::pid_t;
use std::sync::LazyLock;

const UNITY: [&str; 6] = [
    "com.miHoYo.Yuanshen",
    "com.miHoYo.hkrpg",
    "com.tencent.tmgp.sgame",
    "com.miHoYo.Nap",
    "com.yongshi.tenojo.ys",
    "com.tencent.tmgp.speedmobile",
];

const UNNAME: [&str; 3] = [
    "com.tencent.tmgp.pubgmhd",
    "com.netease.yyslscn",
    "com.netease.yyslscn",
];

const MINGCHAO: [&str; 3] = [
    "com.kurogame.mingchao",
    "com.papegames.infinitynikki",
    "com.kurogame.mingchao",
];

pub struct StartArgs<'a> {
    pub task_map: &'a HashMap<pid_t, CompactString>,
    pub controller: &'a mut Controller,
}
type ConfigTuple<'a> = (&'a [&'a str], fn(&mut StartArgs));

pub static PACKAGE_CONFIGS: LazyLock<[ConfigTuple; 3]> = LazyLock::new(|| {
    [
        (&UNNAME[..], policy_unname::start_task),
        (&UNITY[..], policy_unity::start_task),
        (&MINGCHAO[..], policy_ue::start_task),
    ]
});
