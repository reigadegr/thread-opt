use libc::{cpu_set_t, pid_t, sched_setaffinity, CPU_SET, CPU_ZERO};
pub fn bind_thread_to_cpu(tid: &pid_t, cpu_indices: &[u8]) {
    unsafe {
        let mut cpu_set = std::mem::zeroed::<cpu_set_t>();
        CPU_ZERO(&mut cpu_set);
        for &cpu_index in cpu_indices {
            CPU_SET(cpu_index as _, &mut cpu_set);
        }
        let _ = sched_setaffinity(*tid, std::mem::size_of::<cpu_set_t>(), &cpu_set);
    }
}
