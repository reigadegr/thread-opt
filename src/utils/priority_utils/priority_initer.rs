use libc::{pid_t, sched_param};

// 辅助通用函数：初始化 CPU 集
pub const fn create_prio_obj(priority: pid_t) -> sched_param {
    unsafe {
        let mut param: sched_param = core::mem::zeroed();
        param.sched_priority = priority;
        param
    }
}
