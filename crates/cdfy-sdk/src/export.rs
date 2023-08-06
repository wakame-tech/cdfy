use crate::types::*;

#[fp_bindgen_support::fp_export_signature]
pub fn default_state() -> IResult;

#[fp_bindgen_support::fp_export_signature]
pub fn on_event(state: String, event: String) -> IResult;

#[fp_bindgen_support::fp_export_signature]
pub fn plugin_meta() -> PluginMeta;
