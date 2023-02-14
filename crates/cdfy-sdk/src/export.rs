use crate::types::*;

/// fire when elements clicked
#[fp_bindgen_support::fp_export_signature]
pub fn on_click(player_id: String, id: String, state: State) -> State;

/// fire when a room is created
#[fp_bindgen_support::fp_export_signature]
pub fn on_create_room(player_id: String) -> State;

/// fire when join a player
#[fp_bindgen_support::fp_export_signature]
pub fn on_join_player(player_id: String, state: State) -> State;

/// returns plugin meta infomation
#[fp_bindgen_support::fp_export_signature]
pub fn plugin_meta() -> PluginMeta;
