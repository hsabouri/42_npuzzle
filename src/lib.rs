extern crate rand;
#[macro_use]
extern crate rand_derive;
// #[macro_use]
// extern crate lazy_static;
extern crate colored;

// mod translator;
mod parser;
mod generator;
mod node;
mod map;
mod solver;

pub use map::{Map,Point,Heuristic};
pub use node::Node;
// use rand::Rng;
// use std::cmp::Ordering;
use colored::*;
pub use solver::Solver;


// pub fn process(&self, StartNode: Node) {
//     if let Some(mut map) = StartNode.map {
//         map.display(self.size);
//         map.translate_in(&self.solved);
//         println!("Order:");
//         map.display(self.size);
//         // let t = Map::new(self.translate_in(map.content), map.pos, None);
//         // t.display(self.size);
//         // map.first_get_costs(Heuristic::Naive);
//     }
// }

// fn init_solver(size: u16) {
//     unsafe {SOLVER.size = size;}
//     let zero_pos = match size % 2 {
//         0 => size / 2 - 1 + (size / 2) * size,
//         _ => size / 2 + (size / 2) * size,
//     };
//     unsafe {SOLVER.zero_pos = zero_pos};
//     let map = create_solved(size as i16);
//     unsafe {SOLVER.solved = map};
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Rand)]
pub enum Movement {
    Up,
    Down,
    Left,
    Right,
    No,
}

pub fn process(mut start_node: Node) {
    if let Some(ref mut map) = start_node.map {
        map.display();
        map.translate_in();
        println!("Order:");
        map.display();
        // let t = Map::new(generator::translate_in(map.content), map.pos, None);
        // t.display();
        // map.first_get_costs(Heuristic::Naive);
    }
}

pub fn parse(filename: &str, func: Heuristic) -> Result<Node, &'static str> {
    let (vec_spiral, point, size) = match parser::parse(filename) {
        Ok(x) => x,
        Err(msg) => {println!("{}", msg.red()); return Err("Failed to parse")},
    };
    let solver = Solver::new(size, func);
    let map = Map::new(vec_spiral, &solver, point, None);
    Ok(Node::new_from_map(map))
}


pub fn create_random(size: u16, func: Heuristic) -> Result<Node, &'static str> {
    let solver: &Solver = Solver::new(size, func);
    let zero_pos = solver.zero_pos;
    let mut vec_spiral = generator::create_solved_spiral(size as i16); //TODO remove this generation and clone solver
    vec_spiral[zero_pos as usize] = 0;

    let mut map = Map::new(vec_spiral, &solver, Point{x: zero_pos % size, y: zero_pos / size}, None);
    map.shuffle();
    Ok(Node::new_from_map(map))
}
