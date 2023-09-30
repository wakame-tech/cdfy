use crate::types::*;

#[fp_bindgen_support::fp_import_signature]
pub fn debug(message: String);

#[fp_bindgen_support::fp_import_signature]
pub fn rand() -> u32;

/// reserve task and execute returns `task_id`
#[fp_bindgen_support::fp_import_signature]
pub fn reserve(player_id: String, room_id: String, action: String, timeout: u32) -> String;
