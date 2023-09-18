use std::net::{TcpListener, TcpStream};
use sync::core::node;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:80");

    let listener=match listener{
        Ok(listener)=>listener,
        Err(error)=>panic!("Something went wrong starting the listening process : {:?}",error)
    };
    
    let node = node::Node::new().unwrap();

    println!("{:?}", node);

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(stream) => stream,
            Err(error) => panic!("Something went wrong while reading the stream {:?}", error),
        };

        node.handle_stream(stream)
    }
}
