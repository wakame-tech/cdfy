use anyhow::Result;
use cdfy_sdk_core::Event;
use cdfy_server_sdk::reserve;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashSet, VecDeque},
    fmt::Debug,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterState {
    tasks: VecDeque<String>,
    player_ids: HashSet<String>,
    count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    Increment,
    WillIncrement,
}

impl Default for CounterState {
    fn default() -> Self {
        Self {
            tasks: VecDeque::new(),
            player_ids: HashSet::new(),
            count: 0,
        }
    }
}

impl CounterState {
    pub fn on_event(&mut self, event: Event<Action>) -> Result<()> {
        match event {
            Event::OnJoinPlayer { player_id, .. } => {
                self.player_ids.insert(player_id);
            }
            Event::OnLeavePlayer { player_id, .. } => {
                self.player_ids.remove(&player_id);
            }
            Event::OnTask { task_id } | Event::OnCancelTask { task_id } => {
                if let Some(i) = self.tasks.iter().position(|id| id == &task_id) {
                    self.tasks.remove(i);
                }
            }
            Event::Message {
                player_id,
                room_id,
                message: action,
            } => match action {
                Action::Increment => {
                    self.count += 1 + self.player_ids.len();
                }
                Action::WillIncrement => {
                    let task_id = reserve(
                        player_id,
                        room_id,
                        serde_json::to_string(&Action::Increment).unwrap(),
                        3000,
                    );
                    self.tasks.push_back(task_id);
                }
            },
        }
        Ok(())
    }
}
