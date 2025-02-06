use super::affinity_setter::create_cpu_set;
use crate::cgroup::group_info::{get_background_group, get_middle_group, get_top_group};
use libc::cpu_set_t;
use once_cell::sync::Lazy;

// 定义全局变量
pub static TOP_CPU_SET: Lazy<cpu_set_t> = Lazy::new(|| unsafe { create_cpu_set(get_top_group()) });

pub static ONLY7_CPU_SET: Lazy<cpu_set_t> = Lazy::new(|| unsafe { create_cpu_set(&[7]) });

pub static ONLY6_CPU_SET: Lazy<cpu_set_t> = Lazy::new(|| unsafe { create_cpu_set(&[6]) });

pub static MIDDLE_CPU_SET: Lazy<cpu_set_t> =
    Lazy::new(|| unsafe { create_cpu_set(get_middle_group()) });

pub static BACKGROUND_CPU_SET: Lazy<cpu_set_t> =
    Lazy::new(|| unsafe { create_cpu_set(get_background_group()) });

// 通过函数获取全局变量的引用
pub fn get_top_cpu_set() -> &'static cpu_set_t {
    &TOP_CPU_SET
}

pub fn get_only7_cpu_set() -> &'static cpu_set_t {
    &ONLY7_CPU_SET
}

pub fn get_only6_cpu_set() -> &'static cpu_set_t {
    &ONLY6_CPU_SET
}

pub fn get_middle_cpu_set() -> &'static cpu_set_t {
    &MIDDLE_CPU_SET
}

pub fn get_background_cpu_set() -> &'static cpu_set_t {
    &BACKGROUND_CPU_SET
}
