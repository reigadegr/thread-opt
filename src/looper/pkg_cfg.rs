use crate::looper::policy_normal;
use crate::looper::policy_pubg;
use std::sync::LazyLock;
const NORMAL_PACKAGE: [&str; 7] = [
    "com.miHoYo.Yuanshen",
    "com.miHoYo.hkrpg",
    "com.tencent.tmgp.sgame",
    "com.miHoYo.Nap",
    "com.kurogame.mingchao",
    "com.yongshi.tenojo.ys",
    "com.tencent.tmgp.speedmobile",
];

const PUBG_PACKAGE: [&str; 1] = ["com.tencent.tmgp.pubgmhd"];

type ConfigTuple<'a> = (&'a [&'a str], fn(&u32, &str));
pub static PACKAGE_CONFIGS: LazyLock<[ConfigTuple; 2]> = LazyLock::new(|| {
    [
        (
            &NORMAL_PACKAGE[..],
            policy_normal::start_task as fn(&u32, &str),
        ),
        (&PUBG_PACKAGE[..], policy_pubg::start_task as fn(&u32, &str)),
    ]
});
