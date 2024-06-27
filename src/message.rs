use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageData {
    pub from: String,
    pub content: String,
}

impl Clone for MessageData {
    fn clone(&self) -> MessageData {
        Self {
            from: self.from.clone(),
            content: self.content.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Join(String),
    Chat(MessageData),
    Leave(String),
    Poll(String),
    None
}
