use crate::room::{
    broascast::{broadcast, Notification, Notifier},
    redis::RedisRoomStore,
    Room, RoomStore,
};
use anyhow::{Error, Result};
use axum::{
    extract::{Path, WebSocketUpgrade},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::broadcast;
use tower_http::add_extension::AddExtensionLayer;

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

async fn join_room(
    Path((room_id, user_id)): Path<(String, String)>,
    Extension(notifier): Extension<Arc<Notifier>>,
) -> ApiRespnse<Room> {
    tracing::debug!("room {}/join {}", room_id, user_id);
    let store = RedisRoomStore::default();

    let mut room = store.get(room_id.clone()).map_err(into_resp)?;
    room.join(user_id.clone()).map_err(into_resp)?;
    if let Err(e) = notifier.tx.send(Notification::new(
        room_id.clone(),
        user_id.clone(),
        room.clone(),
    )) {
        tracing::error!("{}", e);
    }

    store.set(&room).map_err(into_resp)?;
    Ok(Json(room))
}

async fn delete_room(Path(room_id): Path<String>) -> ApiRespnse<()> {
    tracing::debug!("room {} delete", room_id);
    let store = RedisRoomStore::default();

    store.delete(room_id).map_err(into_resp)?;
    Ok(Json(()))
}

#[derive(Debug, Deserialize)]
struct LoadPluginBody {
    user_id: String,
    plugin_id: String,
}

async fn load_plugin(
    Path(room_id): Path<String>,
    Extension(notifier): Extension<Arc<Notifier>>,
    body: Json<LoadPluginBody>,
) -> ApiRespnse<Room> {
    tracing::debug!("room {}/plugin load {:?}", room_id, body);
    let store = RedisRoomStore::default();

    let mut room = store.get(room_id.clone()).map_err(into_resp)?;
    room.load_plugin(body.plugin_id.clone())
        .map_err(into_resp)?;
    if let Err(e) = notifier.tx.send(Notification::new(
        room_id.clone(),
        body.user_id.clone(),
        room.clone(),
    )) {
        tracing::error!("{}", e);
    }

    store.set(&room).map_err(into_resp)?;
    Ok(Json(room))
}

#[derive(Deserialize)]
pub struct MessageBody {
    user_id: String,
    message: String,
}

async fn plugin_message(
    Path((room_id, plugin_id)): Path<(String, String)>,
    Extension(notifier): Extension<Arc<Notifier>>,
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

    let mut room = store.get(room_id.clone()).map_err(into_resp)?;
    room.message(body.user_id.clone(), plugin_id, body.message.clone())
        .map_err(into_resp)?;

    if let Err(e) = notifier.tx.send(Notification::new(
        room_id.clone(),
        body.user_id.clone(),
        room.clone(),
    )) {
        tracing::error!("{}", e);
    }

    store.set(&room).map_err(into_resp)?;
    Ok(Json(room))
}

async fn event_listener_handler(
    Path((room_id, user_id)): Path<(String, String)>,
    ws: WebSocketUpgrade,
    Extension(notifier): Extension<Arc<Notifier>>,
) -> impl IntoResponse {
    let mut rx = notifier.tx.subscribe();
    ws.on_upgrade(|socket| async move {
        broadcast(socket, &mut rx, room_id, user_id).await;
    })
}

pub fn router() -> Router {
    let (tx, _) = broadcast::channel(100);

    Router::new()
        .route("/rooms", get(list_rooms))
        .route("/rooms/:room_id", get(get_room).post(create_room))
        .route(
            "/rooms/:room_id/listen/:user_id",
            get(event_listener_handler),
        )
        .route("/rooms/:room_id", delete(delete_room))
        .route("/rooms/:room_id/join/:user_id", post(join_room))
        .route("/rooms/:room_id/plugins", post(load_plugin))
        .route(
            "/rooms/:room_id/plugins/:plugin_id/message",
            post(plugin_message),
        )
        .layer(AddExtensionLayer::new(Arc::new(Notifier { tx })))
}
