use crate::room::{create_room, delete_room, get_room, join_room, list_rooms};
use axum::{
    routing::{delete, get, post},
    Router,
};
use std::net::SocketAddr;

pub mod room;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    tracing::debug!("this is a tracing line");

    let app = Router::new()
        .route("/rooms", get(list_rooms))
        .route("/rooms/:room_id/:user_id", post(join_room))
        .route("/rooms/:room_id", get(get_room).post(create_room))
        .route("/rooms/:room_id", delete(delete_room));

    let addr = SocketAddr::from(([127, 0, 0, 1], 1234));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
