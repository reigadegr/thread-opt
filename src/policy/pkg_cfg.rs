use super::usage_top2::{policy_party, policy_top2, policy_ue_t2, policy_unity_t2};
use crate::activity::ActivityUtils;
use libc::pid_t;

// 对于需要取两个重负载线程的游戏，其线程前缀名均为"Thread-"，目前策略是燕云十六声特调
const USAGE_T2: &[&str] = &["com.netease.yyslscn"];

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

pub const PACKAGE_CONFIGS: &[ConfigTuple] = &[
    (USAGE_T2, policy_top2::start_task),
    (UNITY_T2, policy_unity_t2::start_task),
    (PARTY_T2, policy_party::start_task),
    (UE_T2, policy_ue_t2::start_task),
];
