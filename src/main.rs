use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;

use axum::handler::HandlerWithoutStateExt;
use axum::routing::{get, get_service};
use axum::{Router, Server};
use axum_login::axum_sessions::{async_session, SessionLayer};
use axum_login::{memory_store, AuthLayer, AuthUser};
use clap::Parser;
use log::info;
use rand::Rng;
use tokio::sync::RwLock;
use tower::ServiceExt;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

mod auth;
mod error;
mod route;
mod user;

use crate::auth::RequireAuth;
use crate::user::User;

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

    // Initialize session layer
    let secret: [u8; 64] = rand::thread_rng().gen();
    let session = SessionLayer::new(async_session::MemoryStore::new(), &secret).with_secure(false);
    // Initialize auth layer
    let user = User::new("Zakhary".to_string());
    let store = Arc::new(RwLock::new(HashMap::new()));
    store.write().await.insert(user.get_id(), user);
    let store = memory_store::MemoryStore::new(&store);
    let auth = AuthLayer::new(store, &secret);

    // Build our application with a route
    let app = Router::new()
        .route("/", get(route::index))
        .route("/login", get(route::login).post(route::auth))
        .route("/logout", get(route::logout))
        .route("/rsvp", get(route::rsvp).layer(RequireAuth::login()))
        .fallback_service(
            get_service(
                ServeDir::new("www").not_found_service(
                    error::e404
                        .into_service()
                        .map_err(|err| -> io::Error { match err {} }),
                ),
            )
            .handle_error(error::e500),
        )
        .layer(TraceLayer::new_for_http())
        .layer(auth)
        .layer(session);

    // Run it
    let addr = SocketAddr::from(([0; 8], args.port));
    info!("listening on {addr}");
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
