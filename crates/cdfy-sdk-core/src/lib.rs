use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Event {
    OnJoinPlayer {
        player_id: String,
        room_id: String,
    },
    OnLeavePlayer {
        player_id: String,
        room_id: String,
    },
    Message {
        player_id: String,
        room_id: String,
        message: String,
    },
}
