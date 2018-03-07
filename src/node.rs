// mod map;
use super::Map;
use super::Movement;
use std::cmp::Ordering;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Node {
    pub map: Option<Map>,
    pub parent: usize,
    pub movement: Movement,
    pub hash: usize, // Can basically be a weighted addition of the content and the complexity
    pub g: usize,
    pub h: usize,
    pub f: usize,
}

impl Node {
    pub fn new(map: Map, parent: usize, movement: Movement, hash: usize, g: usize, h: usize, f: usize) -> Node {
        Node {
            map: Some(map),
            parent: parent,
            movement: movement,
            hash: hash,
            g: g,
            h: h,
            f: f,
        }
    }

    // pub fn child(&mut self, movement: Movement, parent: usize, solved: &Map) -> Node {
    //     let mut map = self.map.clone().unwrap();

    //     map.child(&movement);
    //     let h = map.get_cost(None, &solved);
    //     Node {
    //         map: Some(map),
    //         parent: parent,
    //         movement: movement,
    //         hash: 0, //TODO
    //         g: self.g + 1,
    //         h: h,
    //         f: self.g + 1 + h,
    //     }
    // }

    // pub fn gen(size: i16) -> (Node, Node) {
    //     let solved = Map::get_solved(size);
    //     solved.display();
    //     let map = Map::gen(size as usize, &solved);
    //     let h = map.get_cost(None, &solved);

    //     (Node {
    //         map: Some(map),
    //         parent: 0,
    //         movement: Movement::No,
    //         hash: 0,
    //         g: 0,
    //         h: h,
    //         f: h,
    //     }, Node {
    //          map: Some(solved),
    //         parent: 0,
    //         movement: Movement::No,
    //         hash: 0,
    //         g: 0,
    //         h: 0,
    //         f: 0,

    //     })
    // }
    pub fn new_from_map(map: Map) -> Node {
        Node {
            map: Some(map),
            parent: 0,
            movement: Movement::No,
            hash: 0,
            g: 0,
            h: 0,
            f: 0
        }
    }
    // pub fn new_solved() -> Node {
    //     Node {
    //         map: Some(Map::new_solved(unsafe {SOLVER.size})),
    //         parent: 0,
    //         movement: Movement::No,
    //         hash: 0,
    //         g: 0,
    //         h: 0,
    //         f: 0
    //     }
    // }
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
