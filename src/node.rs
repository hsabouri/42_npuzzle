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

    pub fn child(&mut self, movement: Movement, parent: usize) -> Node {
        let mut map = self.map.clone().unwrap();

        map.child(&movement);
        let h = map.get_cost();
        Node {
            map: Some(map),
            parent: parent,
            movement: movement,
            g: self.g + 1,
            h: h,
            f: self.g + 1 + h,
        }
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
        self.f.cmp(&other.f)
    }
}
