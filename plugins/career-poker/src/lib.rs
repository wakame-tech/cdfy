use card::Card;
#[cfg(target_arch = "wasm32")]
use cdfy_sdk::{cancel, fp_export_impl, reserve, PluginMeta, State};
#[cfg(not(target_arch = "wasm32"))]
use mock::*;
use serde::{Deserialize, Serialize};
use state::CareerPokerState;

#[cfg(not(target_arch = "wasm32"))]
pub mod mock;

pub mod card;
pub mod deck;
pub mod state;

#[derive(Serialize, Deserialize, Debug)]
pub enum Action {
    Distribute,
    Pass,
    Flush { to: String },
    OneChance { serves: Vec<Card> },
    SelectTrushes { serves: Vec<Card> },
    SelectPasses { serves: Vec<Card> },
    SelectExcluded { serves: Vec<Card> },
    Serve { serves: Vec<Card> },
}

pub fn will_flush(player_id: String, room_id: String, to: String) -> String {
    reserve(
        player_id,
        room_id,
        serde_json::to_string(&Action::Flush { to }).unwrap(),
        5000,
    )
}

pub fn cancel_task(room_id: String, task_id: String) {
    cancel(room_id, task_id);
}

impl Into<CareerPokerState> for State {
    fn into(self) -> CareerPokerState {
        serde_json::from_str(&self.data).unwrap()
    }
}

#[cfg(target_arch = "wasm32")]
#[fp_export_impl(cdfy_sdk)]
pub fn plugin_meta() -> PluginMeta {
    PluginMeta {
        name: env!("CARGO_PKG_NAME").to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    }
}

#[cfg(target_arch = "wasm32")]
#[fp_export_impl(cdfy_sdk)]
pub fn on_create_room(player_id: String, room_id: String) -> State {
    let mut state = CareerPokerState::new(room_id);
    state.players.push(player_id);
    State {
        data: serde_json::to_string(&state).unwrap(),
    }
}

#[cfg(target_arch = "wasm32")]
#[fp_export_impl(cdfy_sdk)]
pub fn on_join_player(player_id: String, _room_id: String, state: State) -> State {
    let mut state: CareerPokerState = state.into();
    state.join(player_id);
    State {
        data: serde_json::to_string(&state).unwrap(),
    }
}

#[cfg(target_arch = "wasm32")]
#[fp_export_impl(cdfy_sdk)]
pub fn on_leave_player(player_id: String, _room_id: String, state: State) -> State {
    let mut state: CareerPokerState = state.into();
    state.leave(player_id);
    State {
        data: serde_json::to_string(&state).unwrap(),
    }
}

#[cfg(target_arch = "wasm32")]
#[fp_export_impl(cdfy_sdk)]
pub fn on_task(_task_id: String, state: State) -> State {
    let mut state: CareerPokerState = serde_json::from_str(&state.data.as_str()).unwrap();
    state.will_flush_task_id = None;
    State {
        data: serde_json::to_string(&state).unwrap(),
    }
}

#[cfg(target_arch = "wasm32")]
#[fp_export_impl(cdfy_sdk)]
pub fn on_cancel_task(_task_id: String, state: State) -> State {
    let mut state: CareerPokerState = serde_json::from_str(&state.data.as_str()).unwrap();
    state.will_flush_task_id = None;
    State {
        data: serde_json::to_string(&state).unwrap(),
    }
}

#[cfg(target_arch = "wasm32")]
#[fp_export_impl(cdfy_sdk)]
pub fn rpc(player_id: String, _room_id: String, state: State, value: String) -> State {
    let mut state: CareerPokerState = serde_json::from_str(&state.data.as_str()).unwrap();
    let action: Action = serde_json::from_str(value.as_str()).unwrap();
    match action {
        Action::Distribute => state.distribute(),
        Action::Pass => state.pass(player_id),
        Action::Flush { to } => state.flush(to),
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
