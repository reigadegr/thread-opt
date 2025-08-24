use super::cpu_set_initer::create_cpu_set;
use crate::cgroup::group_info::{get_background_group, get_middle_group, get_top_group};
use libc::cpu_set_t;

// 定义静态变量的宏
macro_rules! cpuset_once_lazy {
    ($name:ident, $init:expr) => {
        pub static $name: std::sync::LazyLock<cpu_set_t> = std::sync::LazyLock::new(|| create_cpu_set($init));
    };
}

// 定义获取静态变量引用的宏
macro_rules! cpuset_getter {
    ($name:ident, $static_name:ident) => {
        pub fn $name() -> &'static cpu_set_t {
            &$static_name
        }
    };
}

// 使用宏定义静态变量
cpuset_once_lazy!(TOP_CPU_SET, get_top_group());
cpuset_once_lazy!(ONLY7_CPU_SET, &[7]);
cpuset_once_lazy!(ONLY6_CPU_SET, &[6]);
cpuset_once_lazy!(ZERO_SIX_CPU_SET, &[0, 1, 2, 3, 4, 5, 6]);
cpuset_once_lazy!(MIDDLE_CPU_SET, get_middle_group());
cpuset_once_lazy!(BACKGROUND_CPU_SET, get_background_group());
cpuset_once_lazy!(
    BACKGROUND_MIDDLE_CPU_SET,
    &[get_background_group(), get_middle_group()].concat()
);

// 使用宏定义获取函数
cpuset_getter!(get_top_cpu_set, TOP_CPU_SET);
cpuset_getter!(get_only7_cpu_set, ONLY7_CPU_SET);
cpuset_getter!(get_dualo_cpu_set, ONLY6_CPU_SET);
cpuset_getter!(get_zero_six_cpu_set, ZERO_SIX_CPU_SET);
cpuset_getter!(get_middle_cpu_set, MIDDLE_CPU_SET);
cpuset_getter!(get_background_cpu_set, BACKGROUND_CPU_SET);
cpuset_getter!(get_middle_background_cpu_set, BACKGROUND_MIDDLE_CPU_SET);
