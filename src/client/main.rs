use std::net::TcpStream;
use sync::core::node::Node;

fn main() {
    let stream = TcpStream::connect("127.0.0.1:8080");

    let stream = match stream {
        Ok(stream) => stream,
        Err(error) => panic!("Something went wrong starting the stream {}", error),
    };

    let client_node = Node::new().unwrap();

    client_node.handle_stream(stream);

    println!("Hello wordl")
}
