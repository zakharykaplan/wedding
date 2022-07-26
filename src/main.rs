use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::{Router, Server};
use log::info;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Build our application with a route
    let app = Router::new().route("/", get(index));

    // Run it
    let addr = "[::]:3000".parse().unwrap();
    info!("listening on {addr}");
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> impl IntoResponse {
    Html("<h1>Hello, world!</h1>")
}
