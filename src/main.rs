extern crate rand;
mod lib;

fn main() {
    let node = lib::Node::gen(10);
    let child = lib::Node::child(&node, lib::Movement::Right, 0);
    println!("{:#?}", node);
    println!("{:#?}", child);
}
