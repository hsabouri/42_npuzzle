extern crate rand;
#[macro_use]
extern crate rand_derive;
extern crate colored;

mod parser;
mod generator;

use rand::Rng;
use std::cmp::Ordering;
use colored::*;

pub struct solver {
    size: u16
}

static mut SOLVER: solver = solver {
    size: 0
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Rand)]
pub enum Movement {
    Up,
    Down,
    Left,
    Right,
    No,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Heuristic {
    Manhattan,
    Wrong,
    Linear,
    Composit,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Map {
    pub content: Vec<u16>,
    pub pos: Point,
    pub costs: Option<Vec<usize>>,
}

// fn h_wrong(map: &Map, old: Option<&Map>, solved: &Map) -> Vec<u16> {
//     match old {
//         Some(_) => {
//             let unwrappedOld = old.unwrap();
//             let pos = map.pos.x + map.pos.y * map.size;
//             let mut res = unwrappedOld.costs.clone();

//             res[pos] = if res[pos] == solved.content[pos] {0} else {2};
//             res
//         },
//         None => {
//             let mut res = Vec::<u16>::new();

//             for (i, value) in map.content.iter().enumerate() {
//                 res.push(if *value == solved.content[i] {0} else {2});
//             }
//             res
//         }
//     }
// }
fn spiral(w: i16, h: i16, x: i16, y: i16) -> i16 {
    println!("W {} | H {} | x {} | y {}", w ,h ,x ,y);
    if y == 0 {
        x + 1
    } else {
        w + spiral(h - 1, w, y - 1, w - x - 1)
    }
}

impl Map {
    fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        for _ in 0..500 {
            let mov: Movement = rng.gen();
            println!("{:?}", mov);
        }
        // TODO shuffle here

    }

    pub fn new_random(size: u16) -> Map {
        let (solved, pos) = generator::get_solved(size);
        let mut map = Map {content: solved, pos: pos, costs: None};
        map.shuffle();
        map
    }

    pub fn new_solved(size: u16) -> Map {
        let (solved, pos) = generator::get_solved(size);
        Map {content: solved, pos: pos, costs: None}
    }

    pub fn new(content: Vec<u16>, pos: Point, costs: Option<Vec<usize>>) -> Map {
        Map {
            content: content,
            pos: pos,
            costs: costs,
        }
    }

    // pub fn get_costs(&self, old: Option<&Map>, solved: &Map, func: Heuristic) -> Vec<u16> {
    //     match func {
    //         _ => h_wrong(self, old, solved)
    //     }
    // }

    // pub fn get_cost(&self, old: Option<&Map>, solved: &Map) -> usize {
    //     self.get_costs(old, solved, Heuristic::Wrong).iter().fold(0, |acc, &x| acc + x as usize)
    // }
    
    // pub fn child(&mut self, movement: &Movement) {
    //     self.content.swap(self.pos.x + self.pos.y * unsafe {SOLVER.size}, {
    //         match *movement {
    //             Movement::Down => self.pos.x + (self.pos.y - 1) * unsafe {SOLVER.size},
    //             Movement::Up => self.pos.x + (self.pos.y + 1) * unsafe {SOLVER.size},
    //             Movement::Right => (self.pos.x - 1) + self.pos.y * unsafe {SOLVER.size},
    //             Movement::Left => (self.pos.x + 1) + self.pos.y * unsafe {SOLVER.size},
    //             Movement::No => self.pos.x + self.pos.y * unsafe {SOLVER.size}
    //         }
    //     });

    //     self.pos = match *movement {
    //         Movement::Right => Point {x: self.pos.x - 1, y: self.pos.y},
    //         Movement::Left => Point {x: self.pos.x + 1, y: self.pos.y},
    //         Movement::Down => Point {x: self.pos.x, y: self.pos.y - 1},
    //         Movement::Up => Point {x: self.pos.x, y: self.pos.y + 1},
    //         Movement::No => Point {x: self.pos.x, y: self.pos.y},
    //     };
    // }

    // pub fn get_solved(side: i16) -> Map {
    //     let mut map: Vec<usize> = Vec::new();
    //     let size = side * side;
    //     for x in 0..side {
    //         for y in 0..side {
    //             match spiral(side as i16, side as i16, y, x) {
    //                 var if var == size => map.push(0),
    //                 var     => map.push(var as usize)
    //             };
    //         }
    //     }

    //     Map {
    //         content: map,
    //         pos: Point {
    //             x: match side % 2 {
    //                 0 => side as usize / 2 - 1,
    //                 _ => side as usize / 2,
    //             },
    //             y: side as usize / 2
    //         },
    //         size: side as usize,
    //         costs: (0..(size - 1)).map(|_| 0).collect(),
    //     }
    // }

    pub fn display(&self) {
        for y in 0..unsafe {SOLVER.size} {
            let mut to_display = String::from("");
            for x in 0..unsafe {SOLVER.size} {
                to_display.push_str(format!("{:4}", self.content[(x + y * unsafe {SOLVER.size}) as usize]).as_str());
            }
            println!("{}\n", to_display);
        }
    }

    // pub fn gen(size: usize, solved: &Map) -> Map {
    //     let mut topush: Vec<usize> = (0..(size * size)).collect();
    //     let mut pos = Point {x: 0, y: 0};
    //     let content: Vec<usize> = (0..(size * size)).map(|map_id: usize| {
    //         let id = rand::thread_rng().gen_range(0, topush.len());
    //         let res = topush[id];

    //         topush.remove(id);
    //         if res == 0 {
    //             pos = Point {x: map_id % size, y: map_id / size};
    //         }
    //         res
    //     }).collect();
    //     let mut res = Map {
    //         content: content,
    //         pos: pos,
    //         size: size,
    //         costs: (0..(size * size)).collect(),
    //     };
    //     res.costs = res.get_costs(None, solved, Heuristic::Wrong);
    //     res
    // }
}

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
    pub fn new_solved() -> Node {
        Node {
            map: Some(Map::new_solved(unsafe {SOLVER.size})),
            parent: 0,
            movement: Movement::No,
            hash: 0,
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

pub fn parse(filename: &str) -> Result<(Node, Node), &'static str> {
    let (map, size) = match parser::parse(filename) {
        Ok(x) => x,
        Err(msg) => {println!("{}", msg.red()); return Err("Failed to parse")},
    };
    unsafe {SOLVER.size = size;}
    Ok((Node::new_from_map(map), Node::new_solved()))
}

pub fn create_random(size: u16) -> Result<(Node, Node), &'static str> {
    let map = Map::new_random(size);
    unsafe {SOLVER.size = size;}
    Ok((Node::new_from_map(map), Node::new_solved()))
}

pub fn solve(map: Node, solved: Node) {

}
