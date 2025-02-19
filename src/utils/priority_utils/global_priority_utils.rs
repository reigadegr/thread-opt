use super::global_priority::{
    get_background_priority, get_middle_background_priority, get_middle_priority,
    get_only6_priority, get_only7_priority, get_top_priority,
};
use libc::{SCHED_FIFO, pid_t, sched_setscheduler};
// 宏：生成单个线程优先级设置函数
macro_rules! set_thread_priority {
    ($func_name:ident, $priority:expr) => {
        pub fn $func_name(tid: pid_t) {
            unsafe {
                let param = $priority();
                // 设置线程优先级
                let _ = sched_setscheduler(tid, libc::SCHED_RR, param);
            }
        }
    };
}

// 宏：生成多个线程优先级设置函数
macro_rules! set_list_priority {
    ($func_name:ident, $priority:expr) => {
        pub fn $func_name(tids: &[pid_t]) {
            unsafe {
                let param = $priority();
                // 设置线程优先级
                for &tid in tids {
                    let _ = sched_setscheduler(tid, libc::SCHED_RR, param);
                }
            }
        }
    };
}

// 生成单个线程优先级设置函数
set_thread_priority!(set_tid_to_top_priority, get_top_priority);
set_thread_priority!(set_tid_to_only7_priority, get_only7_priority);
set_thread_priority!(set_tid_to_only6_priority, get_only6_priority);
set_thread_priority!(set_tid_to_middle_priority, get_middle_priority);
set_thread_priority!(set_tid_to_background_priority, get_background_priority);

// 生成多个线程优先级设置函数
set_list_priority!(set_list_to_middle_priority, get_middle_priority);
set_list_priority!(set_list_to_background_priority, get_background_priority);
set_list_priority!(
    set_list_to_middle_background_priority,
    get_middle_background_priority
);
