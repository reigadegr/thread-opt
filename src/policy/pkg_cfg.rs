use super::{
    name_match::policies::{policy_sky, policy_ue, policy_unity},
    usage_top1::policies::{
        policy_cocos, policy_ru, policy_top1, policy_ue5, policy_unity_t1, policy_unity_t1_u2,
        policy_wzm,
    },
    usage_top2::{policy_party, policy_top2, policy_ue_t2, policy_unity_t2},
};
use crate::{
    activity::ActivityUtils,
    config::{PROFILE, get_packages},
};
use libc::pid_t;
use once_cell::sync::Lazy;

// 对于普通的Unity游戏
static UNITY: Lazy<&[&str]> = Lazy::new(|| get_packages(PROFILE.unity.packages.clone()));

// 单纯的的线程名匹配，对于ue游戏
const UE: &[&str] = &["com.kurogame.mingchao"];

// 对于需要取一个cputime最大的线程，其线程前缀名为"Thread-"
const UE_T1: &[&str] = &["com.tencent.lzhx", "com.tencent.tmgp.pubgmhd"];

// 需要取一个cputime最大的线程，其线程前缀名为"GameThread"，只有无限暖暖
const UE5_T1: &[&str] = &["com.papegames.infinitynikki"];

// 拉力竞速3
const RU_T1: &[&str] = &["brownmonster.app.game.rushrally3"];

// cod战区，负载最重线程为WZM_Main
const WZM_T1: &[&str] = &["com.activision.callofduty.warzone"];

// 对于三国杀和时空猎人，跟暖暖策略一样，只是线程名不同
const COCOS_T1: &[&str] = &["com.bf.sgs.hdexp", "com.yinhan.hunter.tx"];

// 需要单独把负载最重的unitymain绑定到cpu7
const UNITY_T1: &[&str] = &["com.tencent.tmgp.cod", "com.tencent.tmgp.cf"];

// 需要取一个cputime第二大的线程，其线程前缀名为"Thread-"，且为unity游戏
const UNITY_T1_U2: &[&str] = &["com.tencent.lolm", "com.tencent.tmgp.speedmobile"];

// 对于需要取两个重负载线程的游戏，其线程前缀名均为"Thread-"，目前策略是燕云十六声特调
const USAGE_T2: &[&str] = &["com.netease.yyslscn"];

// 单纯的的线程名匹配，对于光遇游戏
const SKY_T2: &[&str] = &["com.netease.sky"];

// 对于需要取两个重负载线程的游戏，其线程前缀名分别为"Thread-"，"MainThread"，目前策略是蛋仔派对特调
const PARTY_T2: &[&str] = &["com.netease.party"];

// 对于需要取两个重负载线程的游戏，其线程前缀名分别为"UnityMain","Thread-"
const UNITY_T2: &[&str] = &[
    "com.galasports.operablebasketball.mi",
    "com.sofunny.Sausage",
    "com.tencent.jkchess",
];

// 对于需要取两个重负载线程的游戏，其线程前缀名分别为"GameThread","RenderThread"，目前策略只有三角洲
const UE_T2: &[&str] = &["com.tencent.tmgp.dfm"];

pub struct StartArgs<'a> {
    pub activity_utils: &'a mut ActivityUtils,
    pub pid: pid_t,
}

type ConfigTuple = (&'static [&'static str], fn(&mut StartArgs));

// pub static CUST_CONFIGS: Lazy<&[ConfigTuple]> = Lazy::new(|| &[(UNITY, policy_unity::start_task)]);
pub static CUST_CPNFIGS: Lazy<[ConfigTuple; 1]> =
    Lazy::new(|| [(*UNITY, policy_unity::start_task)]);

pub const PACKAGE_CONFIGS: &[ConfigTuple] = &[
    (UE, policy_ue::start_task),
    (UE_T1, policy_top1::start_task),
    (UE5_T1, policy_ue5::start_task),
    (WZM_T1, policy_wzm::start_task),
    (RU_T1, policy_ru::start_task),
    (COCOS_T1, policy_cocos::start_task),
    (UNITY_T1, policy_unity_t1::start_task),
    (UNITY_T1_U2, policy_unity_t1_u2::start_task),
    (USAGE_T2, policy_top2::start_task),
    (UNITY_T2, policy_unity_t2::start_task),
    (PARTY_T2, policy_party::start_task),
    (UE_T2, policy_ue_t2::start_task),
    (SKY_T2, policy_sky::start_task),
];
