use axum::{routing::get, routing::post, Json, Router, extract::State};
use serde_json::json;
use std::sync::Arc;
use crate::models::message::Message;


use crate::state::AppState;


pub fn messages_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", post(send_message))
        .route("/", get(get_messages))
        .with_state(state)
}

// Ajouter un message
async fn send_message(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Message>,
) -> Json<serde_json::Value> {
    let mut messages = state.messages.lock().unwrap();
    messages.push(payload.clone());

    Json(json!({
        "status": "success",
        "message": String::from("Message ajouté"), 
        "data": payload
    }))

}

// Récupérer tous les messages
async fn get_messages(State(state): State<Arc<AppState>>) -> Json<Vec<Message>> {
    let messages = state.messages.lock().unwrap();
    Json(messages.clone())
}
