use crate::{
    cgroup::group_info::{get_background_group, get_middle_group, get_top_group},
    utils::affinity_utils::global_cpu_utils::{
        bind_tid_to_background, bind_tid_to_middle, bind_tid_to_only6, bind_tid_to_only7,
        bind_tid_to_top,
    },
};
use libc::pid_t;

pub fn top_policy(tid: pid_t) {
    bind_tid_to_top(tid);
}

pub fn only6_policy(tid: pid_t) {
    if get_middle_group() == get_background_group() {
        bind_tid_to_only6(tid);
        return;
    }
    bind_tid_to_middle(tid);
}
pub fn only7_policy(tid: pid_t) {
    bind_tid_to_only7(tid);
}
pub fn middle_policy(tid: pid_t) {
    if get_top_group().len() == 4 {
        bind_tid_to_top(tid);
        return;
    }
    bind_tid_to_middle(tid);
}
pub fn background_policy(tid: pid_t) {
    bind_tid_to_background(tid);
}
