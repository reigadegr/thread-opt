use super::common::{CmdType, Policy};
use crate::{config, policy::pkg_cfg::StartArgs, utils::node_reader::write_to_byte};

pub fn start_task(
    args: &mut StartArgs<'_>,
    policy: &config::Policy,
    comm_prefix: &[u8],
    cmd_type: &CmdType,
) {
    if policy.core_closer {
        let _ = write_to_byte(b"/sys/devices/system/cpu/cpu7/online\0", b"0");
    }
    let policy = Policy {
        top: &policy.top,
        dualo: &policy.dualo,
        only7: &policy.only7,
        middle: &policy.middle,
        mono: &policy.mono,
        background: &policy.background,
    };
    super::StartTask::new(args, &policy).start_task(comm_prefix, cmd_type);
    let _ = write_to_byte(b"/sys/devices/system/cpu/cpu7/online\0", b"1");
}
