use crate::policy::pkg_cfg::StartArgs;

pub fn start_task(args: &mut StartArgs<'_>, comm_prefix1: &[u8], comm_prefix2: Option<&[u8]>) {
    super::StartTask::new(args).start_task(comm_prefix1, comm_prefix2);
}
