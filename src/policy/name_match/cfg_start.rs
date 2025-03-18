use super::common::Policy;
use crate::{
    cgroup::group_info::get_top_group, config, policy::pkg_cfg::StartArgs,
    utils::node_reader::lock_val,
};
extern crate alloc;
use alloc::boxed::Box;

pub fn start_task(args: &mut StartArgs<'_>, policy: &config::Policy) {
    let dualo = &policy.dualo;
    let only7 = &policy.only7;
    let mut middle = &policy.middle;
    let mut mono = &policy.mono;

    let empty_box: Box<[heapless::Vec<u8, 16>]> = Box::new([]);

    if policy.core_closer && get_top_group().len() > 1 {
        let _ = lock_val(b"/sys/devices/system/cpu/cpu7/online\0", b"0");
        // dualo = &policy.only7;
        // only7 = &empty_box;
        mono = &empty_box;
        middle = &policy.mono;
    }
    let policy = Policy {
        top: &policy.top,
        dualo,
        only7,
        middle,
        mono,
        background: &policy.background,
    };
    super::StartTask::new(args, &policy).start_task();
    let _ = lock_val(b"/sys/devices/system/cpu/cpu7/online\0", b"1");
}
