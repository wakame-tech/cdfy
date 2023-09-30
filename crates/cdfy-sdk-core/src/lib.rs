use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Event<Message> {
    OnJoinPlayer {
        player_id: String,
        room_id: String,
    },
    OnLeavePlayer {
        player_id: String,
        room_id: String,
    },
    OnCancelTask {
        task_id: String,
    },
    OnTask {
        task_id: String,
    },
    Message {
        player_id: String,
        room_id: String,
        message: Message,
    },
}
