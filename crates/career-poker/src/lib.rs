use card::Card;
// use card::Card;
use cdfy_sdk::{fp_export_impl, PluginMeta, State};
use deck::Deck;
use state::CareerPokerState;

pub mod card;
pub mod deck;
pub mod game;
pub mod state;

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
    let mut state = CareerPokerState::new();
    state.players.push(player_id);
    state.into_state()
}

#[fp_export_impl(cdfy_sdk)]
pub fn on_join_player(player_id: String, state: State) -> State {
    let mut state: CareerPokerState = state.into();
    state.join(player_id);
    state.into_state()
}

#[fp_export_impl(cdfy_sdk)]
pub fn on_leave_player(player_id: String, state: State) -> State {
    let mut state: CareerPokerState = state.into();
    state.leave(player_id);
    state.into_state()
}

#[fp_export_impl(cdfy_sdk)]
pub fn on_click(player_id: String, id: String, state: State, value: String) -> State {
    let mut state: CareerPokerState = state.into();
    match id.as_str() {
        "distribute" => state.distribute(),
        "serve" => {
            let cards: Vec<Card> = serde_json::from_str(&value).unwrap();
            state.serve(player_id, Deck(cards));
        }
        _ => {}
    };
    state.into_state()
}
