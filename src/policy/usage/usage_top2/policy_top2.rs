use super::StartTask;
use crate::policy::pkg_cfg::StartArgs;

pub fn start_task(args: &mut StartArgs<'_>) {
    StartTask::new(args).start_task(b"Thread-");
}
