use cdfy_sdk::{fp_export_impl, PluginMeta, State};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct CounterState {
    actions: Vec<String>,
    count: usize,
}

impl CounterState {
    pub fn new() -> Self {
        Self {
            count: 0,
            actions: vec!["test".to_string()],
        }
    }

    pub fn into_state(&self) -> State {
        State {
            data: serde_json::to_string(&self).unwrap(),
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
pub fn on_create_room(player_id: String) -> State {
    let state = CounterState::new();
    state.into_state()
}

#[fp_export_impl(cdfy_sdk)]
pub fn on_join_player(player_id: String, state: State) -> State {
    state
}

#[fp_export_impl(cdfy_sdk)]
pub fn on_click(player_id: String, id: String, state: State) -> State {
    let mut counter: CounterState = state.into();
    match id.as_str() {
        "test" => {
            counter.count += 2;
        }
        _ => {}
    };
    counter.into_state()
}
