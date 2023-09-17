use std::io::Result;

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
struct Node {
    id: String,
    connections: Vec<&String>,
}

struct Message {
    id: String,
    message_type: MessageType,
    sender_id: Option<String>,
    receiver_id: Option<String>,
}

impl Node {
    fn new() -> Result<Node> {
        Ok(Node {
            id: "1".to_string(),
            connections: Vec::new(),
        })
    }

    fn handleMessage(&mut self, message: Message) {
        match message.message_type {
            MessageType::Connect => {}
            MessageType::AcceptConnect => {
                self.connections.push(&message.sender_id.unwrap());
                println!(
                    "Connection established with node having id : {}",
                    message.sender_id.unwrap()
                );
            }
            MessageType::DeclineConnect => {}
            _ => {}
        }
    }
    fn send_message(&self, reciever_id: String, message_type: MessageType) {}
}

fn main() {
    let node = Node::new().unwrap();
    println!("Hello, world! {:?}", node);
}
