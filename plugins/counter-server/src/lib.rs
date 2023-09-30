use anyhow::Result;
use cdfy_sdk_core::Event;
use cdfy_server_sdk::{fp_export_impl, IResult, PluginMeta};
use game::CounterState;

mod game;

#[fp_export_impl(cdfy_server_sdk)]
pub fn plugin_meta() -> PluginMeta {
    PluginMeta {
        name: env!("CARGO_PKG_NAME").to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    }
}

#[fp_export_impl(cdfy_server_sdk)]
pub fn default_state() -> IResult {
    let state = CounterState::default();
    IResult::Ok(serde_json::to_string(&state).unwrap())
}

fn into_iresult(result: Result<String>) -> IResult {
    match result {
        Ok(t) => IResult::Ok(t),
        Err(e) => IResult::Err(e.to_string()),
    }
}

fn try_on_event(state: String, event: String) -> Result<String> {
    let mut state = serde_json::from_str::<CounterState>(&state)?;
    let event = serde_json::from_str::<Event>(&event)?;
    state.on_event(event)?;
    let new_state = serde_json::to_string(&state)?;
    Ok(new_state)
}

#[fp_export_impl(cdfy_server_sdk)]
pub fn on_event(state: String, event: String) -> IResult {
    into_iresult(try_on_event(state, event))
}
