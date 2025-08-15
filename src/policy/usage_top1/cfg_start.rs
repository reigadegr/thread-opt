use super::common::{CmdType, Policy};
use crate::{config, policy::pkg_cfg::StartArgs};

extern crate alloc;

pub fn start_task(
    args: &mut StartArgs<'_>,
    policy: &config::Policy,
    comm_prefix: &[u8],
    cmd_type: &CmdType,
) {
    let dualo = &policy.dualo;
    let only7 = &policy.only7;
    let middle = &policy.middle;
    let mono = &policy.mono;

    let policy = Policy {
        top: &policy.top,
        dualo,
        only7,
        middle,
        mono,
        background: &policy.background,
    };
    super::StartTask::new(args, &policy).start_task(comm_prefix, cmd_type);
}
