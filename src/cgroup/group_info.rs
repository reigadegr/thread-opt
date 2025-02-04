use super::analysis::{BACKEND_GROUP, MIDDLE_GROUP, TOP_GROUP};
use log::info;

#[inline(always)]
pub fn get_top_group() -> &'static [u8] {
    &TOP_GROUP
}

#[inline(always)]
pub fn get_middle_group() -> &'static [u8] {
    &MIDDLE_GROUP
}

#[inline(always)]
pub fn get_background_group() -> &'static [u8] {
    &BACKEND_GROUP
}

pub fn print_group_core() {
    let top_group = get_top_group();
    let middle_group = get_middle_group();
    let background_group = get_background_group();

    info!("TOP_GROUP: {:?}", top_group);
    info!("MIDDLE_GROUP: {:?}", middle_group);
    info!("BACKEND_GROUP: {:?}", background_group);
}
