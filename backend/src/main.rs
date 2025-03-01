mod routes;
mod state;
mod models;

use axum::{routing::get, Router};
use std::sync::Arc;
use tokio::net::TcpListener;
use state::AppState;
use routes::messages::messages_routes;

// Vérification que l’API tourne
async fn health_check() -> &'static str {
    "🚀 BlockTalk API is running!"
}


#[tokio::main]
async fn main() {
    let state = Arc::new(AppState::new());

    let app = Router::new()
        .route("/", get(health_check))
        .nest("/messages", messages_routes(state.clone()));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("🚀 Serveur lancé sur http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
