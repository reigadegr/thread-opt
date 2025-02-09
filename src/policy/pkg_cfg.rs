use super::name_match::{policy_cocos, policy_ue, policy_unity};
use crate::{
    activity::ActivityUtils,
    cpu_common::Controller,
    policy::usage::{
        usage_top1::{lolm::policy_second, ue5::policy_ue5, unnamed::policy_top1},
        usage_top2::policy_top2,
    },
};
use libc::pid_t;
use once_cell::sync::Lazy;

const UNITY: [&str; 7] = [
    "com.miHoYo.Yuanshen",
    "com.miHoYo.hkrpg",
    "com.tencent.tmgp.sgame",
    "com.miHoYo.Nap",
    "com.yongshi.tenojo.ys",
    "com.tencent.tmgp.speedmobile",
    "com.tencent.KiHan",
];

const UE_USAGE_T1: [&str; 2] = ["com.tencent.lzhx", "com.tencent.tmgp.pubgmhd"];

const USAGE_T2: [&str; 1] = ["com.netease.yyslscn"];

const UE: [&str; 1] = ["com.kurogame.mingchao"];

const UE5: [&str; 1] = ["com.papegames.infinitynikki"];

const COCOS: [&str; 1] = ["com.bf.sgs.hdexp"];

const LOLM: [&str; 1] = ["com.tencent.lolm"];

pub struct StartArgs<'a> {
    pub controller: &'a mut Controller,
    pub activity_utils: &'a mut ActivityUtils,
    pub pid: pid_t,
}

type ConfigTuple<'a> = (&'a [&'a str], fn(&mut StartArgs));

pub static PACKAGE_CONFIGS: Lazy<[ConfigTuple; 7]> = Lazy::new(|| {
    [
        (&UE_USAGE_T1[..], policy_top1::start_task),
        (&USAGE_T2[..], policy_top2::start_task),
        (&UNITY[..], policy_unity::start_task),
        (&UE[..], policy_ue::start_task),
        (&UE5[..], policy_ue5::start_task),
        (&COCOS[..], policy_cocos::start_task),
        (&LOLM[..], policy_second::start_task),
    ]
});
