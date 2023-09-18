use super::{
    data::Data,
    message::{Message, MessageType},
};
use serde_json;
use std::{
    fmt::Debug,
    io::Result,
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

#[derive(Debug)]
pub struct Node {
    pub id: String,
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
        let mut bufreader = BufReader::new(&stream);

        loop {
            let mut line: Vec<u8> = Vec::new();

            match bufreader.read_until(b'\n', &mut line) {
                Ok(0) => {
                    break;
                }
                Ok(_) => {
                    println!("{:?}", line);
                }
                Err(err) => {
                    eprintln!("Error reading from socket : {}", err);
                    break;
                }
            }
        }
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
            MessageType::Send => {
                let data = serde_json::to_string(&self.data);
                let data = match data {
                    Ok(data) => data,
                    Err(error) => panic!("Error occurred serdeing data {:?}", error),
                };
                self.send_data(data)
            }
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
        };
    }

    fn send_data(&self, data: String) {
        println!("{:?}", data)
    }

    fn send_message(&self, reciever_id: String, message_type: MessageType) {
        let message = Message::new(self.id.clone(), reciever_id, message_type);

        println!("{:?}", message)
    }
}
