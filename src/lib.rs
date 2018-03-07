extern crate rand;
#[macro_use]
extern crate rand_derive;
extern crate colored;

mod parser;
mod generator;

use rand::Rng;
use std::cmp::Ordering;
use colored::*;

pub struct Solver {
    size: u16,
    zero_pos: u16
}

static mut SOLVER: Solver = Solver {
    size: 0,
    zero_pos: 0
};

fn init_solver(size: u16) {
    unsafe {SOLVER.size = size;}
    let zero_pos = match size % 2 {
        0 => size / 2 - 1 + (size / 2) * size,
        _ => size / 2 + (size / 2) * size,
    };
    unsafe {SOLVER.zero_pos = zero_pos};
}

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
    Naive,
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

fn from_index_to_value(index: u16) -> Option<u16> {
    let zero_pos = unsafe {SOLVER.zero_pos};

    if index < zero_pos {
        Some(index + 1)
    } else if index > zero_pos {
        Some(index)
    } else if index == zero_pos {
        Some(0)
    } else {
        None
    }
}

impl Map {
    fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            let random_move: Movement = rng.gen();
            if self.can_move(random_move) {
                self.do_move(random_move);
            }
        }
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

    fn can_move(&self, direction: Movement) -> bool {
        let size = unsafe {SOLVER.size};
        match direction {
            Movement::Up => self.pos.y > 0,
            Movement::Down => self.pos.y < (size - 1),
            Movement::Left => self.pos.x > 0,
            Movement::Right => self.pos.x < (size - 1),
            Movement::No => true,
        }
    }

    fn do_move(&mut self, direction: Movement) {
        let size = unsafe {SOLVER.size};
        self.content.swap((self.pos.x + self.pos.y * size) as usize,
        (match direction {
            Movement::Up => self.pos.x + (self.pos.y - 1) * size,
            Movement::Down => self.pos.x + (self.pos.y + 1) * size,
            Movement::Left => (self.pos.x - 1) + self.pos.y * size,
            Movement::Right => (self.pos.x + 1) + self.pos.y * size,
            Movement::No => self.pos.x + self.pos.y * size
        }) as usize
        );

        self.pos = match direction {
            Movement::Up => Point {x: self.pos.x, y: self.pos.y - 1},
            Movement::Down => Point {x: self.pos.x, y: self.pos.y + 1},
            Movement::Left => Point {x: self.pos.x - 1, y: self.pos.y},
            Movement::Right => Point {x: self.pos.x + 1, y: self.pos.y},
            Movement::No => Point {x: self.pos.x, y: self.pos.y},
        };
    }

    fn first_heuristic_naive(&self) -> Vec<u16> {
        let mut res = Vec::<u16>::new();

        for (index, value) in self.content.iter().enumerate() {
            let solved_value = from_index_to_value(index as u16);

            if solved_value.unwrap() == *value {
                res.push(0);
            } else {
                res.push(10);
            }
        }
        res
    }

    pub fn first_get_costs(&self, func: Heuristic) -> Vec<u16> {
        match func {
            //Heuristic::Linear => self.heuristic_linear(solved),
            Heuristic::Naive => self.first_heuristic_naive(),
            _ => self.first_heuristic_naive(),
            //_ => self.heuristic_manhattan(solved),
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
    init_solver(size);
    Ok((Node::new_from_map(map), Node::new_solved()))
}

pub fn create_random(size: u16) -> Result<(Node, Node), &'static str> {
    unsafe {SOLVER.size = size;}
    init_solver(size);
    let map = Map::new_random(size);
    Ok((Node::new_from_map(map), Node::new_solved()))
}

pub fn solve(map_node: Node, solved_node: Node) {
    if let Some(map) = map_node.map {
        map.display();
        map.first_get_costs(Heuristic::Naive);
    }
    println!("Result will be:");
    if let Some(map) = solved_node.map {
        map.display();
    }

}
