use std::{net::SocketAddr, str::FromStr};
use tower_http::cors::{AllowOrigin, CorsLayer};
use tracing_subscriber::EnvFilter;

mod plugin;
mod room;
mod room_api;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_str("cdfy_server=debug").unwrap())
        .init();
    tracing::info!("server started");

    let app = room_api::router().layer(CorsLayer::new().allow_origin(AllowOrigin::any()));

    axum::Server::bind(&SocketAddr::from(([127, 0, 0, 1], 1234)))
        .serve(app.into_make_service())
        .await
        .unwrap();
}
