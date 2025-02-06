use super::global_cpu_set::{
    get_background_cpu_set, get_middle_cpu_set, get_only6_cpu_set, get_only7_cpu_set,
    get_top_cpu_set,
};
use libc::{cpu_set_t, pid_t, sched_setaffinity};

// 绑定单个线程到指定的 CPU 核心
pub fn bind_thread_to_top(tid: pid_t) {
    unsafe {
        let cpu_set = get_top_cpu_set();
        let _ = sched_setaffinity(tid, size_of::<cpu_set_t>(), cpu_set);
    }
}

pub fn bind_thread_to_only7(tid: pid_t) {
    unsafe {
        let cpu_set = get_only7_cpu_set();
        let _ = sched_setaffinity(tid, size_of::<cpu_set_t>(), cpu_set);
    }
}

pub fn bind_thread_to_only6(tid: pid_t) {
    unsafe {
        let cpu_set = get_only6_cpu_set();
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

// 绑定多个线程到指定的 CPU 核心
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
