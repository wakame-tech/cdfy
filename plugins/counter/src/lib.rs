#[cfg(target_arch = "wasm32")]
use anyhow::{anyhow, Result};
#[cfg(target_arch = "wasm32")]
use cdfy_sdk::{cancel, debug, fp_export_impl, reserve, PluginMeta, ResultState, State};
#[cfg(not(target_arch = "wasm32"))]
use mock::*;
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, fmt::Debug};

#[cfg(not(target_arch = "wasm32"))]
pub mod mock;

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Action {
    WillIncrement,
    Cancel,
    Increment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CounterState {
    tasks: VecDeque<String>,
    count: usize,
}

fn from_err<E: Debug>(s: CounterState, r: anyhow::Result<(), E>) -> ResultState {
    match r {
        anyhow::Result::Ok(_) => ResultState::Ok(State {
            data: serde_json::to_string(&s).unwrap(),
        }),
        anyhow::Result::Err(err) => ResultState::Err(format!("{:?}", err)),
    }
}

impl CounterState {
    pub fn new() -> Self {
        Self {
            tasks: VecDeque::new(),
            count: 0,
        }
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
pub fn on_create_room(player_id: String, room_id: String) -> ResultState {
    let state = CounterState::new();
    from_err::<()>(state, Ok(()))
}

#[cfg(target_arch = "wasm32")]
#[fp_export_impl(cdfy_sdk)]
pub fn on_join_player(player_id: String, room_id: String, state: State) -> ResultState {
    let state: Result<CounterState> =
        serde_json::from_str(&state.data).map_err(|e| anyhow!("{}", e));
    let Ok(mut state) = state else {
        return ResultState::Err(state.unwrap_err().to_string());
    };
    from_err::<()>(state, Ok(()))
}

#[cfg(target_arch = "wasm32")]
#[fp_export_impl(cdfy_sdk)]
pub fn on_leave_player(player_id: String, room_id: String, state: State) -> ResultState {
    let state: Result<CounterState> =
        serde_json::from_str(&state.data).map_err(|e| anyhow!("{}", e));
    let Ok(mut state) = state else {
        return ResultState::Err(state.unwrap_err().to_string());
    };
    from_err::<()>(state, Ok(()))
}

#[cfg(target_arch = "wasm32")]
#[fp_export_impl(cdfy_sdk)]
pub fn on_task(task_id: String, state: State) -> ResultState {
    let state: Result<CounterState> =
        serde_json::from_str(&state.data).map_err(|e| anyhow!("{}", e));
    let Ok(mut state) = state else {
        return ResultState::Err(state.unwrap_err().to_string());
    };
    if let Some(i) = state.tasks.iter().position(|id| id == &task_id) {
        state.tasks.remove(i);
    }
    from_err::<()>(state, Ok(()))
}

#[cfg(target_arch = "wasm32")]
#[fp_export_impl(cdfy_sdk)]
pub fn rpc(player_id: String, room_id: String, state: State, value: String) -> ResultState {
    debug(state.data.clone());
    let state: Result<CounterState> =
        serde_json::from_str(&state.data).map_err(|e| anyhow!("{}", e));
    let Ok(mut state) = state else {
        return ResultState::Err(state.unwrap_err().to_string());
    };
    let action: Result<Action> = serde_json::from_str(value.as_str()).map_err(|e| anyhow!("{}", e));
    let Ok(action) = action else {
    return ResultState::Err(action.unwrap_err().to_string());
};
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
    from_err::<()>(state, Ok(()))
}
