use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::runner::PluginRunner;

#[derive(Debug, Serialize, Deserialize)]
pub struct Room {
    room_id: String,
    users: HashSet<String>,
    states: HashMap<String, String>,
}

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
        for (k, state) in self.states.iter_mut() {
            let runner = PluginRunner::new(&self.room_id, &k)?;
            *state = runner.on_join(state.to_string(), user_id.clone())?;
        }
        Ok(())
    }

    pub fn leave(&mut self, user_id: String) -> Result<()> {
        self.users.remove(&user_id);
        for (k, state) in self.states.iter_mut() {
            let runner = PluginRunner::new(&self.room_id, &k)?;
            *state = runner.on_leave(state.to_string(), user_id.clone())?;
        }
        Ok(())
    }

    pub fn load_plugin(&mut self, plugin_id: String) -> Result<()> {
        let runner = PluginRunner::new(&self.room_id, &plugin_id)?;
        let state = runner.load()?;
        self.states.insert(plugin_id, state);
        Ok(())
    }

    pub fn message(&mut self, plugin_id: String, message: String) -> Result<()> {
        let runner = PluginRunner::new(&self.room_id, &plugin_id)?;
        let state = self.get_state(&plugin_id)?;
        let state = runner.message(state, message)?;
        self.update_state(plugin_id, state);
        Ok(())
    }

    pub fn on_task(&mut self, plugin_id: String, task_id: String) -> Result<()> {
        let runner = PluginRunner::new(&self.room_id, &plugin_id)?;
        let state = self.get_state(&plugin_id)?;
        let state = runner.on_task(state, task_id)?;
        self.update_state(plugin_id, state);
        Ok(())
    }

    pub fn on_cancel_task(&mut self, plugin_id: String, task_id: String) -> Result<()> {
        let runner = PluginRunner::new(&self.room_id, &plugin_id)?;
        let state = self.get_state(&plugin_id)?;
        let state = runner.on_cancel_task(state, task_id)?;
        self.update_state(plugin_id, state);
        Ok(())
    }
}
