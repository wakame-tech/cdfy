use anyhow::Result;
use cdfy_sdk_core::Event;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fmt::Debug};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterState {
    player_ids: HashSet<String>,
    count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    Increment,
}

impl Default for CounterState {
    fn default() -> Self {
        Self {
            player_ids: HashSet::new(),
            count: 0,
        }
    }
}

impl CounterState {
    pub fn on_event(&mut self, event: Event<Message>) -> Result<()> {
        match event {
            Event::OnJoinPlayer { player_id, .. } => {
                self.player_ids.insert(player_id);
            }
            Event::OnLeavePlayer { player_id, .. } => {
                self.player_ids.remove(&player_id);
            }
            Event::Message { message, .. } => match message {
                Message::Increment => {
                    self.count += 1 + self.player_ids.len();
                }
            },
        }
        Ok(())
    }
}
