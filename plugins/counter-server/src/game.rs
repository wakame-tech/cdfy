use anyhow::{Error, Result};
use cdfy_sdk_core::Event;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fmt::Debug};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterState {
    player_ids: HashSet<String>,
    count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
    pub fn on_event(&mut self, event: Event) -> Result<()> {
        match event {
            Event::OnJoinPlayer { player_id, .. } => {
                self.player_ids.insert(player_id);
            }
            Event::OnLeavePlayer { player_id, .. } => {
                self.player_ids.remove(&player_id);
            }
            Event::Message { message, .. } => {
                let message = serde_json::from_str(&message).map_err(Error::from)?;
                match message {
                    Message::Increment => {
                        self.count += 1 + self.player_ids.len();
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Message;

    #[test]
    fn a() {
        let message: Message = serde_json::from_str("\"Increment\"").unwrap();
        // println!("{}", serde_json::to_string(&Message::Increment).unwrap());
        assert_eq!(message, Message::Increment);
    }
}
