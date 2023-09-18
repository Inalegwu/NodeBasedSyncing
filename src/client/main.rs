use sync::core::node;

fn main() {
    let node = node::Node::new().unwrap();
    println!("{:?}", node)
}
