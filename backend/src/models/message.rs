use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub sender: String,
    pub content: String,
}
