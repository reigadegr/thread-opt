use libc::{cpu_set_t, pid_t, sched_setaffinity, CPU_SET, CPU_ZERO};

// 辅助通用函数：初始化 CPU 集
pub fn create_cpu_set(cpu_indices: &[u8]) -> cpu_set_t {
    unsafe {
        let mut cpu_set = core::mem::zeroed();
        CPU_ZERO(&mut cpu_set);
        for &cpu_index in cpu_indices {
            CPU_SET(cpu_index as _, &mut cpu_set);
        }
        cpu_set
    }
}

// 绑定单个线程到指定的 CPU 核心
#[allow(dead_code)]
pub fn bind_tid_to_cpu(cpu_indices: &[u8], tid: pid_t) {
    unsafe {
        let cpu_set = create_cpu_set(cpu_indices);
        let _ = sched_setaffinity(tid, core::mem::size_of::<cpu_set_t>(), &cpu_set);
    }
}

// 绑定多个线程到指定的 CPU 核心
#[allow(dead_code)]
pub fn bind_tid_list_to_cgroup(cpu_indices: &[u8], tids: &[pid_t]) {
    unsafe {
        let cpu_set = create_cpu_set(cpu_indices);
        for &tid in tids {
            let _ = sched_setaffinity(tid, core::mem::size_of::<cpu_set_t>(), &cpu_set);
        }
    }
}
