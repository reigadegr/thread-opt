use super::analysis::{BACKEND_GROUP, MIDDLE_GROUP, TOP_GROUP};
use log::info;

pub fn get_top_group() -> &'static [u8] {
    TOP_GROUP.get().map_or(&[7], Box::as_ref)
}

pub fn get_middle_group() -> &'static [u8] {
    MIDDLE_GROUP.get().map_or(&[4, 5, 6], Box::as_ref)
}

pub fn get_background_group() -> &'static [u8] {
    BACKEND_GROUP.get().map_or(&[0, 1, 2, 3], Box::as_ref)
}

pub fn print_group_core() {
    let top_group = get_top_group();
    let middle_group = get_middle_group();
    let background_group = get_background_group();

    info!("TOP_GROUP: {:?}", top_group);
    info!("MIDDLE_GROUP: {:?}", middle_group);
    info!("BACKEND_GROUP: {:?}", background_group);
}
