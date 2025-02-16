use super::{
    name_match::policies::{policy_ue, policy_unity},
    usage_top1::policies::{policy_cocos, policy_codm, policy_second, policy_top1, policy_ue5},
    usage_top2::{policy_party, policy_top2},
};
use crate::activity::ActivityUtils;
use libc::pid_t;
use once_cell::sync::Lazy;

// 对于普通的Unity游戏
const UNITY: [&str; 13] = [
    "com.miHoYo.Yuanshen",
    "com.miHoYo.GenshinImpact",
    "com.miHoYo.ys.bilibili",
    "com.miHoYo.yuanshencb",
    "com.miHoYo.hkrpg",
    "com.miHoYo.hkrpg.bilibili",
    "com.HoYoverse.hkrpgoversea",
    "com.miHoYo.hkrpgcb",
    "com.tencent.tmgp.sgame",
    "com.miHoYo.Nap",
    "com.yongshi.tenojo.ys",
    "com.tencent.tmgp.speedmobile",
    "com.tencent.KiHan",
];

// 单纯的的线程名匹配，对于ue游戏
const UE: [&str; 1] = ["com.kurogame.mingchao"];

// 对于需要取一个cputime最大的线程，其线程前缀名为"Thread-"
const UE_USAGE_T1: [&str; 2] = ["com.tencent.lzhx", "com.tencent.tmgp.pubgmhd"];

// 需要取一个cputime最大的线程，其线程前缀名为"GameThread"，只有无限暖暖
const UE5: [&str; 1] = ["com.papegames.infinitynikki"];

// 对于三国杀，跟暖暖策略一样，只是线程名不同
const COCOS: [&str; 1] = ["com.bf.sgs.hdexp"];

// 需要取一个cputime第二大的线程，其线程前缀名为"Thread-"
const LOLM: [&str; 1] = ["com.tencent.lolm"];

// codm
const CODM: [&str; 1] = ["com.tencent.tmgp.cod"];

// 对于需要取两个重负载线程的游戏，其线程前缀名均为"Thread-"，目前策略是燕云十六声特调
const USAGE_T2: [&str; 1] = ["com.netease.yyslscn"];

// 对于需要取两个重负载线程的游戏，其线程前缀名分别为"Thread-"，"MainThread"，目前策略是蛋仔派对特调
const PARTY_T2: [&str; 1] = ["com.netease.party"];

pub struct StartArgs<'a> {
    pub activity_utils: &'a mut ActivityUtils,
    pub pid: pid_t,
}

type ConfigTuple<'a> = (&'a [&'a str], fn(&mut StartArgs));

pub static PACKAGE_CONFIGS: Lazy<[ConfigTuple; 9]> = Lazy::new(|| {
    [
        (&UE_USAGE_T1[..], policy_top1::start_task),
        (&USAGE_T2[..], policy_top2::start_task),
        (&PARTY_T2[..], policy_party::start_task),
        (&UNITY[..], policy_unity::start_task),
        (&UE[..], policy_ue::start_task),
        (&UE5[..], policy_ue5::start_task),
        (&COCOS[..], policy_cocos::start_task),
        (&LOLM[..], policy_second::start_task),
        (&CODM[..], policy_codm::start_task),
    ]
});
