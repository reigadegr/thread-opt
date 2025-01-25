use super::policy_unname;
use crate::policy::name_match::{policy_mingchao, policy_unity};
use libc::pid_t;
use std::collections::HashMap;
use std::sync::LazyLock;

const UNITY: [&str; 7] = [
    "com.miHoYo.Yuanshen",
    "com.miHoYo.hkrpg",
    "com.tencent.tmgp.sgame",
    "com.miHoYo.Nap",
    "com.yongshi.tenojo.ys",
    "com.tencent.tmgp.speedmobile",
    "com.papegames.infinitynikki",
];

const UNNAME: [&str; 2] = ["com.tencent.tmgp.pubgmhd", "com.netease.yyslscn"];

const MINGCHAO: [&str; 1] = ["com.kurogame.mingchao"];

type ConfigTuple<'a> = (&'a [&'a str], fn(&HashMap<pid_t, String>));
pub static PACKAGE_CONFIGS: LazyLock<[ConfigTuple; 3]> = LazyLock::new(|| {
    [
        (
            &UNITY[..],
            policy_unity::start_task as fn(&HashMap<pid_t, String>),
        ),
        (
            &UNNAME[..],
            policy_unname::start_task as fn(&HashMap<pid_t, String>),
        ),
        (
            &MINGCHAO[..],
            policy_mingchao::start_task as fn(&HashMap<pid_t, String>),
        ),
    ]
});
