use crate::{
    cgroup::group_info::{get_background_group, get_middle_group, get_top_group},
    utils::affinity_utils::global_cpu_utils::{
        bind_list_to_middle, bind_list_to_middle_background, bind_tid_to_background,
        bind_tid_to_dualo, bind_tid_to_middle, bind_tid_to_only7, bind_tid_to_top,
        bind_tid_to_zero_six,
    },
};
use libc::pid_t;
use once_cell::sync::Lazy;

static ONLY7_POLICY_FN: Lazy<fn(pid_t)> = Lazy::new(|| {
    if get_top_group() == [6] {
        bind_tid_to_top
    } else {
        bind_tid_to_only7
    }
});

static ONLY6_POLICY_FN: Lazy<fn(pid_t)> = Lazy::new(|| {
    if get_middle_group() == get_background_group() {
        bind_tid_to_dualo
    } else {
        bind_tid_to_middle
    }
});

static ZERO_SIX_POLICY_FN: Lazy<fn(pid_t)> = Lazy::new(|| {
    if get_top_group() == [6, 7] {
        bind_tid_to_zero_six
    } else {
        bind_tid_to_middle
    }
});

static MODDLE_POLICY_FN: Lazy<fn(pid_t)> = Lazy::new(|| bind_tid_to_middle);

static TID_LIST_T2_FN: Lazy<fn(&[pid_t])> = Lazy::new(|| {
    if get_background_group() == get_middle_group() {
        bind_list_to_middle
    } else {
        bind_list_to_middle_background
    }
});

pub fn top_policy(tid: pid_t) {
    bind_tid_to_top(tid);
}

pub fn dualo_policy(tid: pid_t) {
    ONLY6_POLICY_FN(tid);
}

pub fn only7_policy(tid: pid_t) {
    ONLY7_POLICY_FN(tid);
}

pub fn mono_policy(tid: pid_t) {
    ZERO_SIX_POLICY_FN(tid);
}

pub fn middle_policy(tid: pid_t) {
    MODDLE_POLICY_FN(tid);
}

pub fn background_policy(tid: pid_t) {
    bind_tid_to_background(tid);
}

pub fn tid_list_t2_policy(tids: &[pid_t]) {
    TID_LIST_T2_FN(tids);
}
