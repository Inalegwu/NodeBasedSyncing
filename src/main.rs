use std::io::Result;
use uuid::Uuid;

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
            id: Uuid::new_v4().to_string(),
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
            id: Uuid::new_v4().to_string(),
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
            MessageType::DeclineConnect => todo!(),
            MessageType::Data => todo!(),
            MessageType::WillSendData => todo!(),
            MessageType::WillNotSendData => todo!(),
            MessageType::CanRecieveData => todo!(),
            MessageType::CanNotRecieveData => todo!(),
        }
    }

    fn send_message(&self, message: Message) {
        println!("{:?}", message);
    }

    fn send_data(&self, reciever_id: &String, message_type: MessageType) {
        let mut connection_i_know: &String;

        for i in 0..self.connections.len() {
            let found = self.connections.get(i).unwrap();
            if found == reciever_id {
                println!("found Node with id : {:?}", found);
                connection_i_know = found;
            }
        }

        let message = Message::new(message_type, self.id.clone(), connection_i_know.to_string());

        self.send_message(message.unwrap())
    }

    fn my_connections(&self) {
        println!("{:?}", self.connections);
    }
}

fn main() {
    let node = Node::new().unwrap();

    node.my_connections();

    node.send_data(&"2".to_string(), MessageType::AcceptConnect)
}
