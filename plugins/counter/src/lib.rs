use cdfy_sdk::{fp_export_impl, IResult, PluginMeta};
use cdfy_sdk_support::Event;
use game::{CounterState, Message};
pub mod game;

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
pub fn default_state() -> IResult {
    let state = CounterState::new();
    IResult::Ok(serde_json::to_string(&state).unwrap())
}

#[cfg(target_arch = "wasm32")]
#[fp_export_impl(cdfy_sdk)]
pub fn on_event(state: String, event: String) -> IResult {
    let mut state = match serde_json::from_str::<CounterState>(&state) {
        Ok(s) => s,
        Err(e) => {
            return IResult::Err(e.to_string());
        }
    };
    let event = match serde_json::from_str::<Event<Message>>(&event) {
        Ok(ev) => ev,
        Err(e) => {
            return IResult::Err(e.to_string());
        }
    };
    if let Err(e) = state.on_event(event) {
        return IResult::Err(e.to_string());
    }
    let new_state = serde_json::to_string(&state).unwrap();
    IResult::Ok(new_state)
}
