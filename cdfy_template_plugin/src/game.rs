use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub count: usize,
}

impl Game {
    pub fn new(_player_ids: Vec<String>) -> Self {
        Game { count: 0 }
    }

    pub fn increment(&mut self) {
        self.count += 1;
    }

    pub fn is_finished(&self) -> bool {
        self.count >= 10
    }
}
