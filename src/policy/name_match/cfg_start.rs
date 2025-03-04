use super::common::Policy;
use crate::{config, policy::pkg_cfg::StartArgs};
extern crate alloc;
pub fn start_task(args: &mut StartArgs<'_>, policy: &config::Policy) {
    let policy = Policy {
        top: &policy.top,
        only6: &policy.only6,
        only7: &policy.only7,
        middle: &policy.middle,
        mono: &policy.mono,
        background: &policy.background,
    };
    super::StartTask::new(args, &policy).start_task();
}
