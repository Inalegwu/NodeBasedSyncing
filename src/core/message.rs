use std::io::Result;

#[derive(Debug)]
pub enum MessageType {
    Connect,
    ConnectOk,
}

#[derive(Debug)]
pub struct Message {
    pub id: String,
    pub sender_id: String,
    pub receiver_id: String,
    pub message_type: MessageType,
}

impl Message {
    pub fn new(sender_id: String, receiver_id: String, message_type: MessageType) -> Result<Self> {
        return Ok(Message {
            id: uuid::Uuid::new_v4().to_string(),
            sender_id,
            receiver_id,
            message_type,
        });
    }
}
