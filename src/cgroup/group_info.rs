use log::info;

use super::analysis::{ALL_CPU_INDICES, BACKEND_GROUP, MIDDLE_GROUP, TOP_GROUP};

pub fn get_top_group() -> &'static [u8] {
    &TOP_GROUP
}

pub fn get_middle_group() -> &'static [u8] {
    &MIDDLE_GROUP
}

pub fn get_background_group() -> &'static [u8] {
    &BACKEND_GROUP
}

pub fn get_all_cpu_indices() -> &'static [u8] {
    &ALL_CPU_INDICES
}

pub fn print_group_core() {
    let top_group = get_top_group();
    let middle_group = get_middle_group();
    let background_group = get_background_group();

    info!("TOP_GROUP: {top_group:?}");
    info!("MIDDLE_GROUP: {middle_group:?}");
    info!("BACKEND_GROUP: {background_group:?}");
}
