use super::common::{CmdType, Policy};
use crate::{config, policy::pkg_cfg::StartArgs};

pub fn start_task(
    args: &mut StartArgs<'_>,
    policy: &config::Policy,
    comm_prefix: &[u8],
    cmd_type: &CmdType,
) {
    let policy = Policy {
        top: &policy.top,
        only6: &policy.only6,
        only7: &policy.only7,
        middle: &policy.middle,
        mono: &policy.mono,
        background: &policy.background,
    };
    super::StartTask::new(args, &policy).start_task(comm_prefix, cmd_type);
}
