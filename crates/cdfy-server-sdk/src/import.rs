use crate::types::*;

#[fp_bindgen_support::fp_import_signature]
pub fn debug(message: String);

#[fp_bindgen_support::fp_import_signature]
pub fn rand() -> u32;
