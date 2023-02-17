use card::Card;
use cdfy_sdk::{fp_export_impl, PluginMeta, State};
use serde::{Deserialize, Serialize};
use state::CareerPokerState;

pub mod card;
pub mod deck;
pub mod state;

#[derive(Serialize, Deserialize, Debug)]
pub enum Action {
    Distribute,
    Pass,
    OneChance { serves: Vec<Card> },
    SelectTrushes { serves: Vec<Card> },
    SelectPasses { serves: Vec<Card> },
    SelectExcluded { serves: Vec<Card> },
    Serve { serves: Vec<Card> },
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
pub fn on_create_room(player_id: String, room_id: String) -> State {
    let mut state = CareerPokerState::new();
    state.players.push(player_id);
    State {
        data: serde_json::to_string(&state).unwrap(),
    }
}

#[fp_export_impl(cdfy_sdk)]
pub fn on_join_player(player_id: String, room_id: String, state: State) -> State {
    let mut state: CareerPokerState = state.into();
    state.join(player_id);
    State {
        data: serde_json::to_string(&state).unwrap(),
    }
}

#[fp_export_impl(cdfy_sdk)]
pub fn on_leave_player(player_id: String, room_id: String, state: State) -> State {
    let mut state: CareerPokerState = state.into();
    state.leave(player_id);
    State {
        data: serde_json::to_string(&state).unwrap(),
    }
}

#[fp_export_impl(cdfy_sdk)]
pub fn rpc(player_id: String, room_id: String, state: State, value: String) -> State {
    let mut state: CareerPokerState = serde_json::from_str(&state.data.as_str()).unwrap();
    let action: Action = serde_json::from_str(value.as_str()).unwrap();
    match action {
        Action::Distribute => state.distribute(),
        Action::Pass => state.pass(player_id),
        Action::OneChance { serves } => {
            state.one_chance(player_id, serves);
        }
        Action::SelectTrushes { serves } => {
            state.select_trushes(player_id, serves);
        }
        Action::SelectPasses { serves } => {
            state.select_excluded(player_id, serves);
        }
        Action::SelectExcluded { serves } => {
            state.select_excluded(player_id, serves);
        }
        Action::Serve { serves } => {
            state.serve(player_id, serves);
        }
    };
    State {
        data: serde_json::to_string(&state).unwrap(),
    }
}
