use super::Map;
use super::Movement;
use std::cmp::Ordering;
pub use std::collections::HashMap;

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

    pub fn child(&mut self, movement: Movement, parent: usize, hashmap: &mut HashMap<Vec<u16>, u16>) -> Option<Node> {
        match self.map {
            Some(ref map) => {
                match map.child(&movement) {
                    Some(mut child_map) => {
                        let h = child_map.get_cost();
                        let mut to_push = true;
                        let to_res = match hashmap.get(&child_map.content) {
                            Some(value) => {
                                to_push = false;
                                if *value > self.g + 1 {
                                    true
                                } else {
                                    false
                                }
                            },
                            None => true
                        };
                        if to_push {
                            hashmap.insert(child_map.content.clone(), self.g + 1);
                        }
                        if to_res {
                            Some (Node {
                                map: Some(child_map),
                                parent: parent,
                                movement: movement,
                                g: self.g + 1,
                                h: h,
                                f: self.g + 1 + h,
                            })
                        } else {
                            None
                        }
                    },
                    None => None,
                }
            },
            None => None,
        }
    }

    pub fn get_childs(&mut self, parent: usize, hashmap: &mut HashMap<Vec<u16>, u16>) -> Vec<Box<Node>> {
        let mut res = Vec::<Box<Node>>::new();

        if self.movement != Movement::Down {
            match self.child(Movement::Up, parent, hashmap) {
                Some(node) => {res.push(Box::new(node));},
                None => {},
            }
        }
        if self.movement != Movement::Up {
            match self.child(Movement::Down, parent, hashmap) {
                Some(node) => {res.push(Box::new(node));},
                None => {},
            }
        }
        if self.movement != Movement::Right {
            match self.child(Movement::Left, parent, hashmap) {
                Some(node) => {res.push(Box::new(node));},
                None => {},
            }
        }
        if self.movement != Movement::Left {
            match self.child(Movement::Right, parent, hashmap) {
                Some(node) => {res.push(Box::new(node));},
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
