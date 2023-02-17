use std::collections::VecDeque;

use cdfy_sdk::{cancel, fp_export_impl, reserve, PluginMeta, State};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
enum Action {
    WillIncrement,
    Cancel,
    Increment,
}

#[derive(Serialize, Deserialize)]
struct CounterState {
    tasks: VecDeque<String>,
    count: usize,
}

impl CounterState {
    pub fn new() -> Self {
        Self {
            tasks: VecDeque::new(),
            count: 0,
        }
    }
}

impl Into<CounterState> for State {
    fn into(self) -> CounterState {
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
    let state = CounterState::new();
    State {
        data: serde_json::to_string(&state).unwrap(),
    }
}

#[fp_export_impl(cdfy_sdk)]
pub fn on_join_player(player_id: String, room_id: String, state: State) -> State {
    state
}

#[fp_export_impl(cdfy_sdk)]
pub fn on_leave_player(player_id: String, room_id: String, state: State) -> State {
    state
}

#[fp_export_impl(cdfy_sdk)]
pub fn on_task(task_id: String, state: State) -> State {
    let mut state: CounterState = state.into();
    if let Some(i) = state.tasks.iter().position(|id| id == &task_id) {
        state.tasks.remove(i);
    }
    State {
        data: serde_json::to_string(&state).unwrap(),
    }
}

#[fp_export_impl(cdfy_sdk)]
pub fn rpc(player_id: String, room_id: String, state: State, value: String) -> State {
    let mut state: CounterState = state.into();
    let action: Action = serde_json::from_str(&value).unwrap();
    match action {
        Action::WillIncrement => {
            let task_id = reserve(
                player_id,
                room_id,
                serde_json::to_string(&Action::Increment).unwrap(),
                3000,
            );
            state.tasks.push_back(task_id);
        }
        Action::Cancel => {
            if let Some(task_id) = state.tasks.pop_front() {
                cancel(room_id, task_id);
            }
        }
        Action::Increment => state.count += 1,
    };
    State {
        data: serde_json::to_string(&state).unwrap(),
    }
}
