use super::affinity_setter::create_cpu_set;
use crate::cgroup::group_info::{get_background_group, get_middle_group, get_top_group};
use libc::{cpu_set_t, pid_t, sched_setaffinity};
use once_cell::sync::Lazy;

pub static TOP_CPU_SET: Lazy<cpu_set_t> = Lazy::new(|| unsafe { create_cpu_set(get_top_group()) });

pub static MIDDLE_CPU_SET: Lazy<cpu_set_t> =
    Lazy::new(|| unsafe { create_cpu_set(get_middle_group()) });

pub static BACKGROUND_CPU_SET: Lazy<cpu_set_t> =
    Lazy::new(|| unsafe { create_cpu_set(get_background_group()) });

pub fn get_top_cpu_set() -> &'static cpu_set_t {
    &TOP_CPU_SET
}

pub fn get_middle_cpu_set() -> &'static cpu_set_t {
    &MIDDLE_CPU_SET
}

pub fn get_background_cpu_set() -> &'static cpu_set_t {
    &BACKGROUND_CPU_SET
}

// 绑定单个线程到指定的 CPU 核心
pub fn bind_thread_to_top(tid: pid_t) {
    unsafe {
        let cpu_set = get_top_cpu_set();
        let _ = sched_setaffinity(tid, size_of::<cpu_set_t>(), cpu_set);
    }
}

pub fn bind_thread_to_middle(tid: pid_t) {
    unsafe {
        let cpu_set = get_middle_cpu_set();
        let _ = sched_setaffinity(tid, size_of::<cpu_set_t>(), cpu_set);
    }
}

pub fn bind_thread_to_background(tid: pid_t) {
    unsafe {
        let cpu_set = get_background_cpu_set();
        let _ = sched_setaffinity(tid, size_of::<cpu_set_t>(), cpu_set);
    }
}

pub fn bind_list_to_middle(tids: &[pid_t]) {
    unsafe {
        let cpu_set = get_middle_cpu_set();
        for &tid in tids {
            let _ = sched_setaffinity(tid, size_of::<cpu_set_t>(), cpu_set);
        }
    }
}

pub fn bind_list_to_background(tids: &[pid_t]) {
    unsafe {
        let cpu_set = get_background_cpu_set();
        for &tid in tids {
            let _ = sched_setaffinity(tid, size_of::<cpu_set_t>(), cpu_set);
        }
    }
}
