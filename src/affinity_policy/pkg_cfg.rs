use super::{policy_pubg, policy_unity};
use libc::pid_t;
use std::sync::LazyLock;
const UNITY_PACKAGE: [&str; 8] = [
    "com.miHoYo.Yuanshen",
    "com.miHoYo.hkrpg",
    "com.tencent.tmgp.sgame",
    "com.miHoYo.Nap",
    "com.kurogame.mingchao",
    "com.yongshi.tenojo.ys",
    "com.tencent.tmgp.speedmobile",
    "com.papegames.infinitynikki",
];

const PUBG_PACKAGE: [&str; 2] = ["com.tencent.tmgp.pubgmhd", "com.netease.yyslscn"];

type ConfigTuple<'a> = (&'a [&'a str], fn(pid_t, &str));
pub static PACKAGE_CONFIGS: LazyLock<[ConfigTuple; 2]> = LazyLock::new(|| {
    [
        (
            &UNITY_PACKAGE[..],
            policy_unity::start_task as fn(pid_t, &str),
        ),
        (
            &PUBG_PACKAGE[..],
            policy_pubg::start_task as fn(pid_t, &str),
        ),
    ]
});
