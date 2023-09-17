use std::io::Result;
use std::net::{TcpListener, TcpStream};

#[derive(Debug)]
enum MessageType {
    Connect,
    AcceptConnect,
    DeclineConnect,
    Data,
    WillSendData,
    WillNotSendData,
    CanRecieveData,
    CanNotRecieveData,
}
#[derive(Debug)]
struct Message {
    id: String,
    message_type: MessageType,
    sender_id: String,
    receiver_id: String,
}

impl Message {
    fn new(message_type: MessageType, sender_id: String, receiver_id: String) -> Result<Self> {
        return Ok(Message {
            id: "1".to_string(),
            message_type,
            sender_id,
            receiver_id,
        });
    }
}

#[derive(Debug)]
struct Node {
    id: String,
    connections: Vec<String>,
}

impl Node {
    fn new() -> Result<Node> {
        Ok(Node {
            id: "1".to_string(),
            connections: Vec::new(),
        })
    }

    fn handle_message(&mut self, message: Message) {
        match message.message_type {
            MessageType::Connect => {}
            MessageType::AcceptConnect => {
                self.connections.push(message.sender_id.clone());
                println!(
                    "Connection established with node having id : {}",
                    message.sender_id
                );
            }
            MessageType::DeclineConnect => {}
            MessageType::Data => {}
            MessageType::WillSendData => {}
            MessageType::WillNotSendData => {}
            MessageType::CanRecieveData => {}
            MessageType::CanNotRecieveData => {}
        }
    }
    fn send_message(self, receiver_id: String, message_type: MessageType) {
        let message: Message = Message::new(message_type, self.id, receiver_id).unwrap();
        println!("{:?}", message)
    }

    fn send_data(&self, reciever_id: &String) {
        let mut _connection: &String;
        for i in 0..self.connections.len() {
            let found = self.connections.get(i).unwrap();
            if found == reciever_id {
                println!("found Node with id : {:?}", found);
                _connection = found;
            }
        }
    }
}

fn main() {
    let node = Node::new().unwrap();

    node.send_message("2".to_string(), MessageType::Connect);
}
