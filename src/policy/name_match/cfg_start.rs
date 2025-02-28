use super::common::Policy;
use crate::config;
use crate::policy::pkg_cfg::StartArgs;
extern crate alloc;
use alloc::{fmt::format, sync::Arc, vec::Vec};
pub fn start_task(args: &mut StartArgs<'_>, policy: &config::Policy) {
    let policy = Policy {
        top: policy.top.clone(),
        only6: policy.only6.clone(),
        only7: policy.only7.clone(),
        middle: policy.middle.clone(),
        background: policy.background.clone(),
    };
    super::StartTask::new(args, &policy).start_task();
}
