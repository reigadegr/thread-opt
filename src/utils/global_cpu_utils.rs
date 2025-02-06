use super::global_cpu_set::{
    get_background_cpu_set, get_middle_cpu_set, get_only6_cpu_set, get_only7_cpu_set,
    get_top_cpu_set,
};
use libc::{cpu_set_t, pid_t, sched_setaffinity};
use std::mem::size_of;

// 宏：生成单个线程绑定函数
macro_rules! bind_thread {
    ($func_name:ident, $get_cpu_set:expr) => {
        pub fn $func_name(tid: pid_t) {
            unsafe {
                let cpu_set = $get_cpu_set();
                let _ = sched_setaffinity(tid, size_of::<cpu_set_t>(), cpu_set);
            }
        }
    };
}

// 宏：生成多个线程绑定函数
macro_rules! bind_list {
    ($func_name:ident, $get_cpu_set:expr) => {
        pub fn $func_name(tids: &[pid_t]) {
            unsafe {
                let cpu_set = $get_cpu_set();
                for &tid in tids {
                    let _ = sched_setaffinity(tid, size_of::<cpu_set_t>(), cpu_set);
                }
            }
        }
    };
}

// 生成单个绑定函数
bind_thread!(bind_thread_to_top, get_top_cpu_set);
bind_thread!(bind_thread_to_only7, get_only7_cpu_set);
bind_thread!(bind_thread_to_only6, get_only6_cpu_set);
bind_thread!(bind_thread_to_middle, get_middle_cpu_set);
bind_thread!(bind_thread_to_background, get_background_cpu_set);

// 生成批量绑定函数
bind_list!(bind_list_to_middle, get_middle_cpu_set);
bind_list!(bind_list_to_background, get_background_cpu_set);
