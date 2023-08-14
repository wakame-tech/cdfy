use std::collections::{HashMap, HashSet};

use anyhow::Result;
use axum::{extract::Path, http::StatusCode, Json};
use redis::{Commands, RedisError};
use serde::{Deserialize, Serialize};

static REDIS_ADDR: &str = "redis://127.0.0.1";

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

    pub fn join(&mut self, user_id: String) {
        self.users.insert(user_id);
    }
}

type ApiRespnse<T> = Result<Json<T>, (StatusCode, String)>;

fn into_resp(e: RedisError) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
}

pub async fn create_room(Path(room_id): Path<String>) -> ApiRespnse<String> {
    tracing::debug!("crate_room {}", room_id);
    let client = redis::Client::open(REDIS_ADDR).map_err(into_resp)?;
    let mut con = client.get_connection().map_err(into_resp)?;

    let key = format!("rooms:{}", room_id);
    let room = Room::new(room_id);
    let room = serde_json::to_string(&room).unwrap();
    let _: () = con.set(&key, room).map_err(into_resp)?;
    Ok(Json("ok".to_string()))
}

pub async fn get_room(Path(room_id): Path<String>) -> ApiRespnse<Room> {
    let client = redis::Client::open(REDIS_ADDR).map_err(into_resp)?;
    let mut con = client.get_connection().map_err(into_resp)?;

    let key = format!("rooms:{}", room_id);
    let room: String = con.get(&key).map_err(into_resp)?;
    let room: Room = serde_json::from_str(&room).unwrap();
    Ok(Json(room))
}

pub async fn list_rooms() -> ApiRespnse<Vec<String>> {
    let client = redis::Client::open(REDIS_ADDR).map_err(into_resp)?;
    let mut con = client.get_connection().map_err(into_resp)?;

    let room_keys = con
        .scan_match("rooms:*")
        .map_err(into_resp)?
        .into_iter()
        .collect::<Vec<String>>();

    Ok(Json(room_keys))
}

pub async fn join_room(Path((room_id, user_id)): Path<(String, String)>) -> ApiRespnse<()> {
    let client = redis::Client::open(REDIS_ADDR).map_err(into_resp)?;
    let mut con = client.get_connection().map_err(into_resp)?;
    let key = format!("rooms:{}", room_id);
    let room: String = con.get(&key).map_err(into_resp)?;
    dbg!(&room);
    let mut room: Room = serde_json::from_str(&room).unwrap();
    room.join(user_id);
    let room = serde_json::to_string(&room).unwrap();
    let _: () = con.set(&key, room).map_err(into_resp)?;
    Ok(Json(()))
}

pub async fn delete_room(Path(room_id): Path<String>) -> ApiRespnse<()> {
    let client = redis::Client::open(REDIS_ADDR).map_err(into_resp)?;
    let mut con = client.get_connection().map_err(into_resp)?;
    let key = format!("rooms:{}", room_id);
    let _: () = con.del(key).map_err(into_resp)?;
    Ok(Json(()))
}
