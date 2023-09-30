use crate::plugin::{runner::PluginEventRunner, WasmPlugin};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

pub mod redis;

pub trait RoomStore {
    fn list_ids(&self) -> Result<Vec<String>>;
    fn set(&self, room: &Room) -> Result<()>;
    fn get(&self, room_id: String) -> Result<Room>;
    fn delete(&self, room_id: String) -> Result<()>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Room {
    pub room_id: String,
    users: HashSet<String>,
    states: HashMap<String, String>,
}

const WASM: &[u8] = include_bytes!("../../../../.cache/counter_server.wasm");

impl Room {
    pub fn new(room_id: String) -> Self {
        Self {
            room_id,
            users: HashSet::new(),
            states: HashMap::new(),
        }
    }

    fn get_state(&self, plugin_id: &str) -> Result<String> {
        self.states
            .get(plugin_id)
            .cloned()
            .ok_or(anyhow!("state of {} not found", plugin_id))
    }

    fn update_state(&mut self, plugin_id: String, state: String) {
        self.states.insert(plugin_id, state);
    }

    pub fn join(&mut self, user_id: String) -> Result<()> {
        self.users.insert(user_id.clone());
        for (_, state) in self.states.iter_mut() {
            let plugin = WasmPlugin::new(WASM)?;
            *state = plugin.on_join(state.to_string(), self.room_id.clone(), user_id.clone())?;
        }
        Ok(())
    }

    pub fn leave(&mut self, user_id: String) -> Result<()> {
        self.users.remove(&user_id);
        for (_, state) in self.states.iter_mut() {
            let plugin = WasmPlugin::new(WASM)?;
            *state = plugin.on_leave(state.to_string(), self.room_id.clone(), user_id.clone())?;
        }
        Ok(())
    }

    pub fn load_plugin(&mut self, plugin_id: String) -> Result<()> {
        let plugin = WasmPlugin::new(WASM)?;
        let state = plugin.load()?;
        self.states.insert(plugin_id, state);
        Ok(())
    }

    pub fn message(&mut self, user_id: String, plugin_id: String, message: String) -> Result<()> {
        let plugin = WasmPlugin::new(WASM)?;
        let state = self.get_state(&plugin_id)?;
        let state = plugin.message(state, self.room_id.clone(), user_id.clone(), message)?;
        tracing::debug!("state={}", state);
        self.update_state(plugin_id, state);
        Ok(())
    }
}
