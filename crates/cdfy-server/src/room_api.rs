use crate::room::{redis::RedisRoomStore, Room, RoomStore};
use anyhow::{Error, Result};
use axum::{
    extract::Path,
    http::StatusCode,
    routing::{delete, get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

type ApiRespnse<T> = Result<Json<T>, (StatusCode, Json<InternalError>)>;

#[derive(Serialize)]
struct InternalError {
    error: String,
}

fn into_resp(e: Error) -> (StatusCode, Json<InternalError>) {
    tracing::error!("{}", e);
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        InternalError {
            error: e.to_string(),
        }
        .into(),
    )
}

async fn create_room(Path(room_id): Path<String>) -> ApiRespnse<Room> {
    tracing::debug!("room {}/create", room_id);
    let store = RedisRoomStore::default();

    let room = Room::new(room_id);

    store.set(&room).map_err(into_resp)?;
    Ok(Json(room))
}

async fn get_room(Path(room_id): Path<String>) -> ApiRespnse<Room> {
    let store = RedisRoomStore::default();

    let room = store.get(room_id).map_err(into_resp)?;
    Ok(Json(room))
}

async fn list_rooms() -> ApiRespnse<Vec<String>> {
    let store = RedisRoomStore::default();

    let room_keys = store.list_ids().map_err(into_resp)?;
    Ok(Json(room_keys))
}

async fn join_room(Path((room_id, user_id)): Path<(String, String)>) -> ApiRespnse<Room> {
    tracing::debug!("room {}/join {}", room_id, user_id);
    let store = RedisRoomStore::default();

    let mut room = store.get(room_id).map_err(into_resp)?;
    room.join(user_id).map_err(into_resp)?;

    store.set(&room).map_err(into_resp)?;
    Ok(Json(room))
}

async fn delete_room(Path(room_id): Path<String>) -> ApiRespnse<()> {
    tracing::debug!("room {} delete", room_id);
    let store = RedisRoomStore::default();

    store.delete(room_id).map_err(into_resp)?;
    Ok(Json(()))
}

async fn load_plugin(Path((room_id, plugin_id)): Path<(String, String)>) -> ApiRespnse<Room> {
    tracing::debug!("room {}/plugin {} load", room_id, plugin_id);
    let store = RedisRoomStore::default();

    let mut room = store.get(room_id).map_err(into_resp)?;
    room.load_plugin(plugin_id).map_err(into_resp)?;

    store.set(&room).map_err(into_resp)?;
    Ok(Json(room))
}

#[derive(Deserialize)]
pub struct MessageBody {
    user_id: String,
    message: String,
}

async fn message_plugin(
    Path((room_id, plugin_id)): Path<(String, String)>,
    body: Json<MessageBody>,
) -> ApiRespnse<Room> {
    tracing::debug!(
        "room {}/user {}/plugin {} message {}",
        room_id,
        body.user_id,
        plugin_id,
        body.message,
    );
    let store = RedisRoomStore::default();

    let mut room = store.get(room_id).map_err(into_resp)?;
    room.message(body.user_id.clone(), plugin_id, body.message.clone())
        .map_err(into_resp)?;
    store.set(&room).map_err(into_resp)?;
    Ok(Json(room))
}

pub fn router() -> Router {
    Router::new()
        .route("/rooms", get(list_rooms))
        .route("/rooms/:room_id", get(get_room).post(create_room))
        .route("/rooms/:room_id", delete(delete_room))
        .route("/rooms/:room_id/join/:user_id", post(join_room))
        .route("/rooms/:room_id/plugins/:plugin_id", post(load_plugin))
        .route(
            "/rooms/:room_id/plugins/:plugin_id/message",
            post(message_plugin),
        )
}
