use super::message::{Message, MessageType};
use std::{io::Result, net::TcpStream};

#[derive(Debug)]
pub struct Node {
    id: String,
    connections: Vec<String>,
}

impl Node {
    pub fn new() -> Result<Node> {
        Ok(Node {
            id: uuid::Uuid::new_v4().to_string(),
            connections: Vec::new(),
        })
    }

    pub fn handle_stream(&self, stream: TcpStream) {}

    fn handle_message(&mut self, message: Message) {
        match message.message_type {
            MessageType::Connect => self.send_message(message.sender_id, MessageType::ConnectOk),
            MessageType::ConnectOk => {
                self.connections.push(message.sender_id);
            }
        }
    }

    fn send_message(&self, reciever_id: String, message_type: MessageType) {
        let message = Message::new(self.id.clone(), reciever_id, message_type);

        println!("{:?}", message)
    }
}
