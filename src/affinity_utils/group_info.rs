use crate::affinity_utils::analysis::{BACKEND_GROUP, MIDDLE_GROUP, TOP_GROUP};
use log::info;

pub fn get_top_group<'a>() -> &'a [u8] {
    let rs = match TOP_GROUP.get() {
        Some(rs) => rs,
        None => return &[7],
    };
    let rs: &[u8] = Box::as_ref(rs);
    rs
}

pub fn get_middle_group<'a>() -> &'a [u8] {
    let rs = match MIDDLE_GROUP.get() {
        Some(rs) => rs,
        None => return &[4, 5, 6],
    };
    let rs: &[u8] = Box::as_ref(rs);
    rs
}

pub fn get_background_group<'a>() -> &'a [u8] {
    let rs = match BACKEND_GROUP.get() {
        Some(rs) => rs,
        None => return &[0, 1, 2, 3],
    };
    let rs: &[u8] = Box::as_ref(rs);
    rs
}

pub fn print_group_core() {
    let top_group = get_top_group();
    let middle_group = get_middle_group();
    let background_group = get_background_group();

    info!("TOP_GROUP: {:?}", top_group);
    info!("MIDDLE_GROUP: {:?}", middle_group);
    info!("BACKEND_GROUP: {:?}", background_group);
}
