extern crate rand;
mod lib;

fn main() {
    let node = lib::Node::gen(20);
    node.1.map.display();
}
