use crate::room_api::{
    create_room, delete_room, get_room, join_room, list_rooms, load_plugin, message_plugin,
};
use axum::{
    routing::{delete, get, post},
    Router,
};
use std::net::SocketAddr;

pub mod room;
pub mod room_api;
pub mod runner;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    tracing::debug!("this is a tracing line");

    let app = Router::new()
        .route("/rooms", get(list_rooms))
        .route("/rooms/:room_id", get(get_room).post(create_room))
        .route("/rooms/:room_id", delete(delete_room))
        .route("/rooms/:room_id/join/:user_id", post(join_room))
        .route("/rooms/:room_id/:plugin_id", post(load_plugin))
        .route("/rooms/:room_id/:plugin_id/message", post(message_plugin));

    let addr = SocketAddr::from(([127, 0, 0, 1], 1234));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
