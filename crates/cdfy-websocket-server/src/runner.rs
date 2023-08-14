use anyhow::{anyhow, Result};
use cdfy_runtime::spec::{bindings::Runtime, types::IResult};
use cdfy_sdk_support::Event;

pub struct PluginRunner {
    room_id: String,
    plugin_id: String,
    runtime: Runtime,
}

impl PluginRunner {
    pub fn new(room_id: &str, plugin_id: &str) -> Result<Self> {
        let wasm = include_bytes!("../../../.cache/counter.wasm");
        Ok(Self {
            room_id: room_id.to_string(),
            plugin_id: plugin_id.to_string(),
            runtime: Runtime::new(wasm)?,
        })
    }

    pub fn load(&self) -> Result<String> {
        let state = self.runtime.default_state()?;
        match state {
            IResult::Ok(state) => Ok(state),
            IResult::Err(e) => Err(anyhow!("{}", e)),
        }
    }

    pub fn message(&self, state: String, message: String) -> Result<String> {
        let state = self.runtime.on_event(state, message)?;
        match state {
            IResult::Ok(state) => Ok(state),
            IResult::Err(err) => Err(anyhow!("{}", err)),
        }
    }

    pub fn on_join(&self, state: String, player_id: String) -> Result<String> {
        let event = Event::<()>::OnJoinPlayer {
            player_id,
            room_id: self.room_id.to_string(),
        };
        let event = serde_json::to_string(&event)?;
        let state = self.runtime.on_event(state, event)?;
        match state {
            IResult::Ok(state) => Ok(state),
            IResult::Err(e) => Err(anyhow!("{}", e)),
        }
    }

    pub fn on_leave(&self, state: String, player_id: String) -> Result<String> {
        let event = Event::<()>::OnLeavePlayer {
            player_id,
            room_id: self.room_id.to_string(),
        };
        let event = serde_json::to_string(&event)?;
        let state = self.runtime.on_event(state, event)?;
        match state {
            IResult::Ok(state) => Ok(state),
            IResult::Err(e) => Err(anyhow!("{}", e)),
        }
    }

    pub fn on_task(&self, state: String, task_id: String) -> Result<String> {
        let event = Event::<()>::OnTask { task_id };
        let event = serde_json::to_string(&event)?;
        let state = self.runtime.on_event(state, event)?;
        match state {
            IResult::Ok(state) => Ok(state),
            IResult::Err(e) => Err(anyhow!("{}", e)),
        }
    }

    pub fn on_cancel_task(&self, state: String, task_id: String) -> Result<String> {
        let event = Event::<()>::OnCancelTask { task_id };
        let event = serde_json::to_string(&event)?;
        let state = self.runtime.on_event(state, event)?;
        match state {
            IResult::Ok(state) => Ok(state),
            IResult::Err(e) => Err(anyhow!("{}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use cdfy_sdk_support::Event;
    use serde::Serialize;

    #[derive(Debug, Serialize)]
    enum TestMessage {
        Hoge,
    }

    #[test]
    fn test_message() {
        let e = Event::<TestMessage>::Message {
            player_id: "u".to_string(),
            room_id: "a".to_string(),
            message: TestMessage::Hoge,
        };
        let e = serde_json::to_string(&e).unwrap();

        println!("{}", e);
    }
}
