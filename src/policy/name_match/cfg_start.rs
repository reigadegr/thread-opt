use super::common::Policy;
use crate::{
    cgroup::group_info::get_top_group, config, policy::pkg_cfg::StartArgs,
    utils::node_reader::write_to_byte,
};
extern crate alloc;
use alloc::boxed::Box;

pub fn start_task(args: &mut StartArgs<'_>, policy: &config::Policy) {
    let mut dualo = &policy.dualo;
    let mut only7 = &policy.only7;
    let empty_box: Box<[heapless::Vec<u8, 16>]> = Box::new([]);

    if policy.core_closer && get_top_group().len() > 1 {
        let _ = write_to_byte(b"/sys/devices/system/cpu/cpu7/online\0", b"0");
        dualo = &policy.only7;
        only7 = &empty_box;
    }
    let policy = Policy {
        top: &policy.top,
        dualo,
        only7,
        middle: &policy.middle,
        mono: &policy.mono,
        background: &policy.background,
    };
    super::StartTask::new(args, &policy).start_task();
    let _ = write_to_byte(b"/sys/devices/system/cpu/cpu7/online\0", b"1");
}
