use super::{Movement, Node};

pub struct Solved {
    pub memory: usize,
    pub complexity: usize,
    pub sequence: Vec<Movement>,
    pub start_node: Node,
}
