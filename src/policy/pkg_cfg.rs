use crate::activity::ActivityUtils;
use libc::pid_t;

pub struct StartArgs<'a> {
    pub activity_utils: &'a mut ActivityUtils,
    pub pid: pid_t,
}
