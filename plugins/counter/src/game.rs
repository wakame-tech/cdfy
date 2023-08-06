use anyhow::Result;
use cdfy_sdk_support::{builtin::reserve, Event};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashSet, VecDeque},
    fmt::Debug,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    WillIncrement,
    Increment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterState {
    tasks: VecDeque<String>,
    player_ids: HashSet<String>,
    count: usize,
}

impl CounterState {
    pub fn new() -> Self {
        Self {
            tasks: VecDeque::new(),
            player_ids: HashSet::new(),
            count: 0,
        }
    }

    pub fn on_event(&mut self, event: Event<Message>) -> Result<()> {
        match event {
            Event::OnJoinPlayer { player_id, room_id } => {
                self.player_ids.insert(player_id);
                Ok(())
            }
            Event::OnLeavePlayer { player_id, room_id } => {
                self.player_ids.remove(&player_id);
                Ok(())
            }
            Event::OnTask { task_id } | Event::OnCancelTask { task_id } => {
                if let Some(i) = self.tasks.iter().position(|id| id == &task_id) {
                    self.tasks.remove(i);
                }
                Ok(())
            }
            Event::Message {
                player_id,
                room_id,
                message,
            } => match message {
                Message::Increment => {
                    self.count += self.player_ids.len();
                    Ok(())
                }
                Message::WillIncrement => {
                    let task_id = reserve(
                        player_id,
                        room_id,
                        serde_json::to_string(&Message::Increment).unwrap(),
                        3000,
                    );
                    self.tasks.push_back(task_id);
                    Ok(())
                }
            },
        }
    }
}
