use super::Map;
use super::Movement;
use std::cmp::Ordering;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Node {
    pub map: Option<Map>,
    pub parent: usize,
    pub movement: Movement,
    pub g: u16,
    pub h: u16,
    pub f: u16,
}

impl Node {
    pub fn new(map: Map, parent: usize, movement: Movement, g: u16, h: u16, f: u16) -> Node {
        Node {
            map: Some(map),
            parent: parent,
            movement: movement,
            g: g,
            h: h,
            f: f,
        }
    }

    pub fn child(&mut self, movement: Movement, parent: usize) -> Option<Node> {
        match self.map {
            Some(ref map) => {
                match map.child(&movement) {
                    Some(mut child_map) => {
                        let h = child_map.get_cost();
                        Some (Node {
                            map: Some(child_map),
                            parent: parent,
                            movement: movement,
                            g: self.g + 1,
                            h: h,
                            f: self.g + 1 + h,
                        })
                    },
                    None => None,
                }
            },
            None => None,
        }
    }

    pub fn get_childs(&mut self, parent: usize) -> Vec<Node> {
        let mut res = Vec::<Node>::new();

        if self.movement != Movement::Down {
            match self.child(Movement::Up, parent) {
                Some(node) => {res.push(node);},
                None => {},
            }
        }
        if self.movement != Movement::Up {
            match self.child(Movement::Down, parent) {
                Some(node) => {res.push(node);},
                None => {},
            }
        }
        if self.movement != Movement::Right {
            match self.child(Movement::Left, parent) {
                Some(node) => {res.push(node);},
                None => {},
            }
        }
        if self.movement != Movement::Left {
            match self.child(Movement::Right, parent) {
                Some(node) => {res.push(node);},
                None => {},
            }
        }
        self.map = None;
        res
    }

    pub fn new_from_map(map: Map) -> Node {
        Node {
            map: Some(map),
            parent: 0,
            movement: Movement::No,
            g: 0,
            h: 0,
            f: 0
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        other.f.cmp(&self.f)
    }
}
