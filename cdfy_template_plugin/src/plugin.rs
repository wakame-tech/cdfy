use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct GameConfig {
    pub player_ids: Vec<String>,
}

#[derive(Deserialize)]
pub struct LiveEvent {
    pub player_id: String,
    pub event_name: String,
    pub value: HashMap<String, String>,
}

#[derive(Deserialize)]
pub struct RenderConfig {
    pub player_id: String,
}
