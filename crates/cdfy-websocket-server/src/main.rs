use crate::room_api::{
    create_room, delete_room, get_room, join_room, list_rooms, load_plugin, message_plugin,
};
use axum::{
    http::HeaderValue,
    routing::{delete, get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

pub mod room;
pub mod room_api;
pub mod runner;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    tracing::info!("server started");

    let origins = ["http://127.0.0.1:8080".parse::<HeaderValue>().unwrap()];

    let app = Router::new()
        .route("/rooms", get(list_rooms))
        .route("/rooms/:room_id", get(get_room).post(create_room))
        .route("/rooms/:room_id", delete(delete_room))
        .route("/rooms/:room_id/join/:user_id", post(join_room))
        .route("/rooms/:room_id/:plugin_id", post(load_plugin))
        .route("/rooms/:room_id/:plugin_id/message", post(message_plugin))
        .layer(CorsLayer::new().allow_origin(origins));

    let addr = SocketAddr::from(([127, 0, 0, 1], 1234));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
