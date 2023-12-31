use std::net::TcpListener;
use sync::core::node::Node;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080");

    let listener = match listener {
        Ok(listener) => listener,
        Err(error) => panic!(
            "Something went wrong starting the listening process : {:?}",
            error
        ),
    };

    let node = Node::new().unwrap();

    println!(
        "Server Node with ID {} started listening on {:?}",
        node.id,
        listener.local_addr().unwrap()
    );

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(stream) => stream,
            Err(error) => panic!("Something went wrong while reading the stream {:?}", error),
        };
        node.handle_stream(stream)
    }
}
