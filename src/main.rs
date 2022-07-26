use std::io;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get_service;
use axum::{Router, Server};
use log::info;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Build our application with a route
    let app = Router::new()
        .fallback(get_service(ServeDir::new("www")).handle_error(e404))
        .layer(TraceLayer::new_for_http());

    // Run it
    let addr = "[::]:3000".parse().unwrap();
    info!("listening on {addr}");
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn e404(err: io::Error) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, format!("404: Not Found: {err}"))
}
