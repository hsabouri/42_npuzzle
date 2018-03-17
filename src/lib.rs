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
mod solved;

pub use std::collections::HashMap;
pub use map::{Map,Point,Heuristic};
pub use node::Node;
// use rand::Rng;
// use std::cmp::Ordering;
use colored::*;
pub use solver::Solver;
pub use solved::Solved;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Rand, Hash)]
pub enum Movement {
    Up,
    Down,
    Left,
    Right,
    No,
}

fn push_sorted(openset: &mut Vec<Node>, node: Node) {
    let index = openset.binary_search(&node).unwrap_or_else(|e| e);
    openset.insert(index, node);
}

pub fn process(mut start_node: Node) -> Result<Solved, &'static str> {
    let mut closeset = Vec::<Node>::new();
    let mut openset  = Vec::<Node>::new();
    let mut hashmap: HashMap<Vec<u16>, u16> = HashMap::new();
    let mut complextity: usize = 0;
    let mut memory: usize = 0;
    let h: u16;

    if let Some(ref mut map) = start_node.map {
        map.display();
        println!("\n");
        map.translate_in();
        map.check_validity()?;
        map.set_first_costs();
        map.display();
        h = map.get_cost();
    } else {
        h = 0;
        // TODO if here we should abort nooo ?
    }

    start_node.h = h;
    start_node.f = h;
    openset.push(start_node);
    loop {
        if let Some(last) = closeset.last() {
            if last.h == 0 {
                break;
            }
        }
        let mut node = openset.pop().unwrap();
        //println!("{:?} {:?}", node.g, node.h);
        let index = closeset.len();
        let mut childs = node.get_childs(index, &mut hashmap);
        closeset.push(node);
        while childs.len() > 0 {
            let child = childs.remove(0);
            push_sorted(&mut openset, child);
            complextity += 1;
        }
        if openset.len() + closeset.len() + hashmap.len() > memory {
            memory = openset.len() + closeset.len() + hashmap.len();
        }
    }
    let mut sequence = Vec::<Movement>::new();
    let end = closeset.pop().unwrap();
    sequence.push(end.movement);
    let mut index = end.parent;
    let mut moves = 0;
    while index != 0 {
        let node = closeset.remove(index);
        index = node.parent;
        sequence.push(node.movement);
        moves += 1;
    }
    sequence.reverse();
    //println!("{:#?}", openset);
    Ok(Solved {
        memory: memory,
        complexity: complextity,
        sequence: sequence,
    })
}

pub fn parse(filename: &str, func: Heuristic, boost: u16) -> Result<Node, &'static str> {
    let (vec_spiral, point, size) = match parser::parse(filename) {
        Ok(x) => x,
        Err(msg) => {println!("{}", msg.red()); return Err("Failed to parse")},
    };
    let solver = Solver::new(size, func, boost);
    let map = Map::new(vec_spiral, &solver, point, None);
    Ok(Node::new_from_map(map))
}

pub fn create_random(size: u16, func: Heuristic, boost: u16) -> Result<Node, &'static str> {
    let solver: &Solver = Solver::new(size, func, boost);
    let zero_index = solver.zero_index;
    let mut vec_spiral = generator::create_solved_spiral(size as i16); //TODO remove this generation and clone solver
    vec_spiral[zero_index as usize] = 0;

    let mut map = Map::new(vec_spiral, &solver, Point{x: zero_index % size, y: zero_index / size}, None);
    map.shuffle();
    Ok(Node::new_from_map(map))
}
