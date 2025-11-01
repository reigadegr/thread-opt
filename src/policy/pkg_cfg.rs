use crate::activity::ActivityUtils;

pub struct StartArgs<'a> {
    pub activity_utils: &'a mut ActivityUtils,
    pub pid: i32,
}
