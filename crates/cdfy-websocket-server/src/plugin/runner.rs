use crate::plugin::WasmPlugin;
use anyhow::{anyhow, Result};
use cdfy_runtime::spec::types::IResult;
use cdfy_sdk_core::Event;

fn to_result(r: IResult) -> Result<String> {
    match r {
        IResult::Ok(state) => Ok(state),
        IResult::Err(e) => Err(anyhow!("{}", e)),
    }
}

pub trait PluginEventRunner {
    type State;

    fn load(&self) -> Result<Self::State>;
    fn message(&self, state: Self::State, room_id: String, message: String) -> Result<Self::State>;
    fn on_join(
        &self,
        state: Self::State,
        room_id: String,
        player_id: String,
    ) -> Result<Self::State>;
    fn on_leave(
        &self,
        state: Self::State,
        room_id: String,
        player_id: String,
    ) -> Result<Self::State>;
    fn on_task(&self, state: Self::State, room_id: String, task_id: String) -> Result<Self::State>;
    fn on_cancel_task(
        &self,
        state: Self::State,
        room_id: String,
        task_id: String,
    ) -> Result<Self::State>;
}

impl PluginEventRunner for WasmPlugin {
    type State = String;

    fn load(&self) -> Result<Self::State> {
        let state = self.runtime.default_state()?;
        to_result(state)
    }

    fn message(
        &self,
        state: Self::State,
        _room_id: String,
        message: String,
    ) -> Result<Self::State> {
        let state = self.runtime.on_event(state, message)?;
        to_result(state)
    }

    fn on_join(
        &self,
        state: Self::State,
        room_id: String,
        player_id: String,
    ) -> Result<Self::State> {
        let event = Event::<()>::OnJoinPlayer { player_id, room_id };
        let event = serde_json::to_string(&event)?;
        let state = self.runtime.on_event(state, event)?;
        to_result(state)
    }

    fn on_leave(
        &self,
        state: Self::State,
        room_id: String,
        player_id: String,
    ) -> Result<Self::State> {
        let event = Event::<()>::OnLeavePlayer { player_id, room_id };
        let event = serde_json::to_string(&event)?;
        let state = self.runtime.on_event(state, event)?;
        to_result(state)
    }

    fn on_task(&self, state: Self::State, room_id: String, task_id: String) -> Result<Self::State> {
        todo!()
    }

    fn on_cancel_task(
        &self,
        state: Self::State,
        room_id: String,
        task_id: String,
    ) -> Result<Self::State> {
        todo!()
    }
}
