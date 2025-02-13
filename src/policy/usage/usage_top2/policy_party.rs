use crate::policy::pkg_cfg::StartArgs;

pub fn start_task(args: &mut StartArgs<'_>) {
    super::StartTask::new(args).start_task(b"MainThread", Some(b"Thread-"));
}
