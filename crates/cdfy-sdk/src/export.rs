use crate::types::*;

#[fp_bindgen_support::fp_export_signature]
pub fn on_cancel_task(task_id: String, state: State) -> ResultState;

/// fire when a room is created
#[fp_bindgen_support::fp_export_signature]
pub fn on_create_room(player_id: String, room_id: String) -> ResultState;

/// fire when join a player
#[fp_bindgen_support::fp_export_signature]
pub fn on_join_player(player_id: String, room_id: String, state: State) -> ResultState;

/// fire when leave a player
#[fp_bindgen_support::fp_export_signature]
pub fn on_leave_player(player_id: String, room_id: String, state: State) -> ResultState;

/// fire when task has been executed
#[fp_bindgen_support::fp_export_signature]
pub fn on_task(task_id: String, state: State) -> ResultState;

/// returns plugin meta infomation
#[fp_bindgen_support::fp_export_signature]
pub fn plugin_meta() -> PluginMeta;

#[fp_bindgen_support::fp_export_signature]
pub fn rpc(player_id: String, room_id: String, state: State, value: String) -> ResultState;
