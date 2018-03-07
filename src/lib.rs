extern crate rand;
#[macro_use]
extern crate rand_derive;
extern crate colored;

mod parser;
mod generator;
mod node;
mod map;

use map::Map;
use node::Node;
// use rand::Rng;
// use std::cmp::Ordering;
use colored::*;

pub struct Solver {
    size: u16,
    sq_size: u16,
    zero_pos: u16,
    solved: Vec<u16>
}

fn spiral(w: i16, h: i16, x: i16, y: i16) -> u16 {
    match y {
        0 => (x + 1) as u16,
        y => w as u16 + spiral(h - 1, w, y - 1, w - x - 1)
    }
}

fn create_solved(size: i16) -> Vec<u16> {
    let mut map: Vec<u16> = Vec::new();
    for x in 0..size {
        for y in 0..size {
            map.push(spiral(size, size, y, x));
        }
    }
    map
}

impl Solver {
    pub fn new(size: u16) -> Solver {
        let ret = Solver {
            size: size,
            sq_size: size * size,
            zero_pos: (size / 2) * (size + 1) + size % 2 - 1,
            solved: create_solved(size as i16)
        };
        ret.solved[ret.zero_pos as usize] = 0;
        ret
    }

    // pub fn init(&self) {
    // }

    pub fn process(&self, StartNode: Node) {
        if let Some(map) = StartNode.map {
            map.display();
            let t = Map::new(generator::translate_in(map.content), map.pos, None);
            println!("Order:");
            t.display();
            // map.first_get_costs(Heuristic::Naive);
        }

    }
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Heuristic {
    Manhattan,
    Naive,
    Linear,
    Composit,
}



// fn from_index_to_value(index: u16) -> u16 {
//     let zero_pos = unsafe {SOLVER.zero_pos};

//     if index < zero_pos {
//         index + 1
//     } else if index > zero_pos {
//         index
//     } else if index == zero_pos {
//         0
//     } else {
//         panic!("lolilol")
//             // None
//     }
// }

pub fn parse(filename: &str) -> Result<(Solver, Node), &'static str> {
    let (map, size) = match parser::parse(filename) {
        Ok(x) => x,
        Err(msg) => {println!("{}", msg.red()); return Err("Failed to parse")},
    };
    // init_solver(size);
    Ok((Solver::new(size), Node::new_from_map(map)))
}

pub fn create_random(size: u16) -> Result<(Solver, Node), &'static str> {
    // unsafe {SOLVER.size = size;}
    // init_solver(size);
    let map = Map::new_random(size);
    // Ok((Node::new_from_map(map), Node::new_solved()))
    Ok((Solver::new(size), Node::new_from_map(map)))
}

// pub fn solve(map_node: Node, solved_node: Node) {
//     generator::Spiral::init(unsafe {SOLVER.size as usize});
//     if let Some(map) = map_node.map {
//         map.display();
//         let t = Map::new(generator::translate_in(map.content), map.pos, None);
//         println!("Order:");
//         t.display();
//         // map.first_get_costs(Heuristic::Naive);
//     }
//     println!("Result will be:");
//     if let Some(map) = solved_node.map {
//         map.display();
//         let t = Map::new(generator::translate_in(map.content), map.pos, None);
//         println!("Order:");
//         t.display();
//     }

// }
