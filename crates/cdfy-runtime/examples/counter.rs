use anyhow::Result;
use cdfy_runtime::spec::{bindings::Runtime, types::IResult};
use cdfy_sdk_support::Event;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CounterState {
    tasks: VecDeque<String>,
    count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Action {
    WillIncrement,
    Increment,
}

fn main() -> Result<()> {
    let wasm = include_bytes!("../../../.cache/counter.wasm");
    let runtime = Runtime::new(wasm)?;

    let state = CounterState {
        tasks: VecDeque::new(),
        count: 0,
    };
    let state = serde_json::to_string(&state).unwrap();
    let event = Event::Message {
        player_id: "player_id".to_string(),
        room_id: "room_id".to_string(),
        message: Action::Increment,
    };
    let event = serde_json::to_string(&event).unwrap();
    if let IResult::Ok(new_state) = runtime.on_event(state, event)? {
        let new_state: CounterState = serde_json::from_str(&new_state)?;
        dbg!(new_state);
    }
    Ok(())
}
