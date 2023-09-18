use super::{
    data::Data,
    message::{Message, MessageType},
};
use std::{fmt::Debug, io::BufReader, io::Result, net::TcpStream};

#[derive(Debug)]
pub struct Node {
    id: String,
    connections: Vec<String>,
    data: Vec<Data>,
}

impl Node {
    pub fn new() -> Result<Node> {
        Ok(Node {
            id: uuid::Uuid::new_v4().to_string(),
            connections: Vec::new(),
            data: Vec::new(),
        })
    }

    pub fn handle_stream(&self, stream: TcpStream) {
        let bufReader = BufReader::new(&stream);
    }

    fn handle_message(&mut self, message: Message) {
        match message.message_type {
            MessageType::Connect => self.send_message(message.sender_id, MessageType::ConnectOk),
            MessageType::ConnectOk => {
                self.connections.push(message.sender_id);
                println!(
                    "Node updated, {} connections active",
                    self.connections.len()
                )
            }
            MessageType::Send => self.send_data(&self.data),
            MessageType::SendOk => todo!(),
            MessageType::Disconnect => {
                for i in 0..self.connections.len() {
                    let found = self.connections.get(i).unwrap();
                    if found == &message.sender_id {
                        self.connections.remove(i);
                    }
                }
                println!("Node updated , {} connections left", self.connections.len())
            }
            MessageType::DisconnectOk => (),
        }
    }

    fn send_data(&self, data: &Vec<Data>) {
        println!("{:?}", data)
    }

    fn send_message(&self, reciever_id: String, message_type: MessageType) {
        let message = Message::new(self.id.clone(), reciever_id, message_type);

        println!("{:?}", message)
    }
}
