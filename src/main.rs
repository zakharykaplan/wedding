use std::io;
use std::net::SocketAddr;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get_service;
use axum::{Router, Server};
use clap::Parser;
use log::info;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

/// Hannah & Zakhary's wedding server.
#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Port to listen for connections.
    #[arg(short, long)]
    #[arg(default_value_t = 3000)]
    #[arg(env = "PORT")]
    port: u16,
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    // Parse args
    let args = Args::parse();

    // Build our application with a route
    let app = Router::new()
        .fallback(get_service(ServeDir::new("www")).handle_error(e404))
        .layer(TraceLayer::new_for_http());

    // Run it
    let addr = SocketAddr::from(([0; 8], args.port));
    info!("listening on {addr}");
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn e404(err: io::Error) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, format!("404: Not Found: {err}"))
}
