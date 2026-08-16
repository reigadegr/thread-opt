use libc::{CPU_SET, CPU_ZERO, cpu_set_t, sched_setaffinity};

// 辅助通用函数：初始化 CPU 集
// 约束：不允许绑定到 cpu0（预留系统核心）
pub fn create_cpu_set(cpu_indices: &[u8]) -> cpu_set_t {
    unsafe {
        let mut cpu_set = core::mem::zeroed();
        CPU_ZERO(&mut cpu_set);
        let mut has_cpu = false;
        for &cpu_index in cpu_indices {
            if cpu_index != 0 {
                CPU_SET(cpu_index as _, &mut cpu_set);
                has_cpu = true;
            }
        }
        // 传入列表非空但全部为 cpu0 被过滤时，回退到 cpu1，
        // 避免空 cpu_set 使 sched_setaffinity 返回 EINVAL 而导致绑定静默失效
        if !has_cpu && !cpu_indices.is_empty() {
            CPU_SET(1, &mut cpu_set);
        }
        cpu_set
    }
}

// 绑定单个线程到指定的 CPU 核心
#[allow(dead_code)]
pub fn bind_tid_to_cpu(cpu_indices: &[u8], tid: i32) {
    unsafe {
        let cpu_set = create_cpu_set(cpu_indices);
        let _ = sched_setaffinity(tid, core::mem::size_of::<cpu_set_t>(), &raw const cpu_set);
    }
}

// 绑定多个线程到指定的 CPU 核心
#[allow(dead_code)]
pub fn bind_tid_list_to_cgroup(cpu_indices: &[u8], tids: &[i32]) {
    unsafe {
        let cpu_set = create_cpu_set(cpu_indices);
        for &tid in tids {
            let _ = sched_setaffinity(tid, core::mem::size_of::<cpu_set_t>(), &raw const cpu_set);
        }
    }
}

#[cfg(test)]
mod tests {
    use libc::CPU_ISSET;

    use super::create_cpu_set;

    fn is_set(cpu: usize, set: &libc::cpu_set_t) -> bool {
        unsafe { CPU_ISSET(cpu, set) }
    }

    #[test]
    fn filters_out_cpu0_from_mixed_list() {
        let set = create_cpu_set(&[0, 3, 5]);
        assert!(!is_set(0, &set));
        assert!(is_set(3, &set));
        assert!(is_set(5, &set));
    }

    #[test]
    fn keeps_normal_cpus() {
        let set = create_cpu_set(&[2, 4]);
        assert!(is_set(2, &set));
        assert!(is_set(4, &set));
        assert!(!is_set(0, &set));
        assert!(!is_set(1, &set));
    }

    #[test]
    fn falls_back_to_cpu1_when_only_cpu0() {
        let set = create_cpu_set(&[0]);
        assert!(!is_set(0, &set));
        assert!(is_set(1, &set));
    }

    #[test]
    fn empty_list_stays_empty() {
        let set = create_cpu_set(&[]);
        assert!(!is_set(0, &set));
        assert!(!is_set(1, &set));
    }
}
