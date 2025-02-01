use super::{
    name_match::{policy_ue, policy_unity},
    usage_top1::policy_unname1,
    usage_top2::policy_unname2,
};
use crate::activity::ActivityUtils;
use crate::cpu_common::Controller;
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

const UNNAME1: [&str; 3] = [
    "com.tencent.tmgp.pubgmhd",
    "com.tencent.tmgp.pubgmhd",
    "com.tencent.tmgp.pubgmhd",
];

const UNNAME2: [&str; 3] = [
    "com.netease.yyslscn",
    "com.netease.yyslscn",
    "com.netease.yyslscn",
];

const MINGCHAO: [&str; 3] = [
    "com.kurogame.mingchao",
    "com.papegames.infinitynikki",
    "com.kurogame.mingchao",
];

pub struct StartArgs<'a> {
    // pub task_map: &'a HashMap<pid_t, CompactString>,
    pub controller: &'a mut Controller,
    pub activity_utils: &'a mut ActivityUtils,
    pub pid: &'a mut pid_t,
}
type ConfigTuple<'a> = (&'a [&'a str], fn(&mut StartArgs));

pub static PACKAGE_CONFIGS: LazyLock<[ConfigTuple; 4]> = LazyLock::new(|| {
    [
        (&UNNAME1[..], policy_unname1::start_task),
        (&UNNAME2[..], policy_unname2::start_task),
        (&UNITY[..], policy_unity::start_task),
        (&MINGCHAO[..], policy_ue::start_task),
    ]
});
