use card::Card;
// use card::Card;
use cdfy_sdk::{fp_export_impl, PluginMeta, State};
use deck::Deck;
// use deck::Deck;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod card;
pub mod deck;
pub mod game;

#[derive(Serialize, Deserialize)]
struct CareerPokerState {
    actions: Vec<String>,
    fields: HashMap<String, Deck>,
}

impl CareerPokerState {
    pub fn new() -> Self {
        Self {
            actions: vec![
                "serve".to_string(),
                "pass".to_string(),
                "distribute".to_string(),
            ],
            fields: HashMap::new(),
        }
    }

    pub fn into_state(&self) -> State {
        State {
            data: serde_json::to_string(&self).unwrap(),
        }
    }
}

impl Into<CareerPokerState> for State {
    fn into(self) -> CareerPokerState {
        serde_json::from_str(&self.data.as_str()).unwrap()
    }
}

#[fp_export_impl(cdfy_sdk)]
pub fn plugin_meta() -> PluginMeta {
    PluginMeta {
        name: env!("CARGO_PKG_NAME").to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    }
}

#[fp_export_impl(cdfy_sdk)]
pub fn on_create_room(player_id: String) -> State {
    let state = CareerPokerState::new();
    state.into_state()
}

#[fp_export_impl(cdfy_sdk)]
pub fn on_join_player(player_id: String, state: State) -> State {
    let mut state: CareerPokerState = state.into();
    state
        .fields
        .insert(player_id, Deck(vec![Card::from("Ah"), Card::from("As")]));
    state.into_state()
}

#[fp_export_impl(cdfy_sdk)]
pub fn on_click(player_id: String, id: String, state: State) -> State {
    let mut state: CareerPokerState = state.into();
    match id.as_str() {
        "distribute" => {
            if let Some(deck) = state.fields.get_mut(&player_id) {
                deck.0.push(Card::from("2s"));
            }
        }
        "serve" => {
            if let Some(deck) = state.fields.get_mut(&player_id) {
                deck.0.clear();
            }
        }
        _ => {}
    };
    state.into_state()
}
