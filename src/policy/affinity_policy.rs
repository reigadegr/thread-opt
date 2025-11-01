use crate::{
    cgroup::group_info::{get_background_group, get_middle_group, get_top_group},
    utils::affinity_utils::global_cpu_utils::{
        bind_list_to_middle, bind_list_to_middle_background, bind_tid_to_background,
        bind_tid_to_dualo, bind_tid_to_middle, bind_tid_to_only7, bind_tid_to_top,
        bind_tid_to_zero_six,
    },
};
use std::sync::LazyLock;

static ONLY7_POLICY_FN: LazyLock<fn(i32)> = LazyLock::new(|| {
    if get_top_group() == [6] {
        return bind_tid_to_top;
    }
    if get_middle_group() == get_background_group() {
        return bind_tid_to_dualo;
    }

    bind_tid_to_only7
});

static ONLY6_POLICY_FN: LazyLock<fn(i32)> = LazyLock::new(|| {
    if get_middle_group() == get_background_group() {
        bind_tid_to_only7
    } else {
        bind_tid_to_middle
    }
});

static ZERO_SIX_POLICY_FN: LazyLock<fn(i32)> = LazyLock::new(|| {
    if get_top_group() == [6, 7] {
        bind_tid_to_zero_six
    } else {
        bind_tid_to_middle
    }
});

static MODDLE_POLICY_FN: LazyLock<fn(i32)> = LazyLock::new(|| bind_tid_to_middle);

static TID_LIST_T2_FN: LazyLock<fn(&[i32])> = LazyLock::new(|| {
    if get_background_group() == get_middle_group() {
        bind_list_to_middle
    } else {
        bind_list_to_middle_background
    }
});

pub fn top_policy(tid: i32) {
    bind_tid_to_top(tid);
}

pub fn dualo_policy(tid: i32) {
    ONLY6_POLICY_FN(tid);
}

pub fn only7_policy(tid: i32) {
    ONLY7_POLICY_FN(tid);
}

pub fn mono_policy(tid: i32) {
    ZERO_SIX_POLICY_FN(tid);
}

pub fn middle_policy(tid: i32) {
    MODDLE_POLICY_FN(tid);
}

pub fn background_policy(tid: i32) {
    bind_tid_to_background(tid);
}

pub fn tid_list_t2_policy(tids: &[i32]) {
    TID_LIST_T2_FN(tids);
}
