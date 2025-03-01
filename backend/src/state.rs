use std::sync::{Arc, Mutex};
use crate::models::message::Message;



#[derive(Clone)]
pub struct AppState {
    pub messages: Arc<Mutex<Vec<Message>>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            messages: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
