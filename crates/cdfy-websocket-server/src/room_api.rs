use crate::room::Room;
use anyhow::Result;
use axum::{extract::Path, http::StatusCode, Json};
use redis::{Commands, RedisError};

static REDIS_ADDR: &str = "redis://127.0.0.1";

type ApiRespnse<T> = Result<Json<T>, (StatusCode, String)>;

fn into_resp(e: RedisError) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
}

pub async fn create_room(Path(room_id): Path<String>) -> ApiRespnse<Room> {
    tracing::debug!("crate_room {}", room_id);
    let client = redis::Client::open(REDIS_ADDR).map_err(into_resp)?;
    let mut con = client.get_connection().map_err(into_resp)?;

    let key = format!("rooms:{}", room_id);
    let room = Room::new(room_id);
    let room_json = serde_json::to_string(&room).unwrap();
    let _: () = con.set(&key, room_json).map_err(into_resp)?;
    Ok(Json(room))
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

pub async fn join_room(Path((room_id, user_id)): Path<(String, String)>) -> ApiRespnse<Room> {
    let client = redis::Client::open(REDIS_ADDR).map_err(into_resp)?;
    let mut con = client.get_connection().map_err(into_resp)?;
    let key = format!("rooms:{}", room_id);
    let room: String = con.get(&key).map_err(into_resp)?;
    let mut room: Room = serde_json::from_str(&room).unwrap();
    room.join(user_id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let room_json = serde_json::to_string(&room).unwrap();
    let _: () = con.set(&key, room_json).map_err(into_resp)?;
    Ok(Json(room))
}

pub async fn delete_room(Path(room_id): Path<String>) -> ApiRespnse<()> {
    let client = redis::Client::open(REDIS_ADDR).map_err(into_resp)?;
    let mut con = client.get_connection().map_err(into_resp)?;
    let key = format!("rooms:{}", room_id);
    let _: () = con.del(&key).map_err(into_resp)?;
    Ok(Json(()))
}

pub async fn load_plugin(Path((room_id, plugin_id)): Path<(String, String)>) -> ApiRespnse<Room> {
    tracing::info!("room {} load {}", room_id, plugin_id);
    let client = redis::Client::open(REDIS_ADDR).map_err(into_resp)?;
    let mut con = client.get_connection().map_err(into_resp)?;

    let key = format!("rooms:{}", room_id);
    let room: String = con.get(&key).map_err(into_resp)?;
    let mut room: Room = serde_json::from_str(&room)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    room.load_plugin(plugin_id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let room_json = serde_json::to_string(&room).unwrap();
    let _: () = con.set(&key, room_json).map_err(into_resp)?;
    Ok(Json(room))
}

pub async fn message_plugin(
    Path((room_id, plugin_id)): Path<(String, String)>,
    message: String,
) -> ApiRespnse<Room> {
    tracing::info!("room {} message", room_id);
    let client = redis::Client::open(REDIS_ADDR).map_err(into_resp)?;
    let mut con = client.get_connection().map_err(into_resp)?;

    let key = format!("rooms:{}", room_id);
    let room: String = con.get(&key).map_err(into_resp)?;
    let mut room: Room = serde_json::from_str(&room)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    room.message(plugin_id, message)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let room_str = serde_json::to_string(&room).unwrap();
    let _: () = con.set(&key, room_str).map_err(into_resp)?;
    Ok(Json(room))
}
