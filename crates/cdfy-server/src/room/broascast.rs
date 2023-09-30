use super::Room;
use axum::extract::ws::{self, WebSocket};
use futures::{SinkExt, StreamExt};
use tokio::sync::broadcast;

#[derive(Debug, Clone)]
pub struct Notification {
    pub room_id: String,
    pub user_id: String,
    pub room: Room,
}

impl Notification {
    pub fn new(room_id: String, user_id: String, room: Room) -> Self {
        Self {
            room_id,
            user_id,
            room,
        }
    }
}

#[derive(Debug)]
pub struct Notifier {
    pub tx: broadcast::Sender<Notification>,
}

pub async fn broadcast(
    socket: WebSocket,
    rx: &mut broadcast::Receiver<Notification>,
    room_id: String,
    user_id: String,
) {
    let (mut sender, _) = socket.split();
    while let Ok(n) = rx.recv().await {
        if n.room_id == room_id && n.user_id == user_id {
            continue;
        }
        let room = serde_json::to_string(&n.room).unwrap();
        if let Err(e) = sender.send(ws::Message::Text(room)).await {
            tracing::error!("{}", e);
        }
    }
    let _ = sender.close().await;
}
