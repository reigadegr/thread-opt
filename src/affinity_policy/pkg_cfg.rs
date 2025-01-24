use super::{policy_mingchao, policy_pubg, policy_unity};
use libc::pid_t;
use std::sync::LazyLock;
const UNITY_PACKAGE: [&str; 7] = [
    "com.miHoYo.Yuanshen",
    "com.miHoYo.hkrpg",
    "com.tencent.tmgp.sgame",
    "com.miHoYo.Nap",
    "com.yongshi.tenojo.ys",
    "com.tencent.tmgp.speedmobile",
    "com.papegames.infinitynikki",
];

const PUBG_PACKAGE: [&str; 2] = ["com.tencent.tmgp.pubgmhd", "com.netease.yyslscn"];

const MINGCHAO_PACKAGE: [&str; 1] = ["com.kurogame.mingchao"];

type ConfigTuple<'a> = (&'a [&'a str], fn(pid_t, &str));
pub static PACKAGE_CONFIGS: LazyLock<[ConfigTuple; 3]> = LazyLock::new(|| {
    [
        (
            &UNITY_PACKAGE[..],
            policy_unity::start_task as fn(pid_t, &str),
        ),
        (
            &PUBG_PACKAGE[..],
            policy_pubg::start_task as fn(pid_t, &str),
        ),
        (
            &MINGCHAO_PACKAGE[..],
            policy_mingchao::start_task as fn(pid_t, &str),
        ),
    ]
});
