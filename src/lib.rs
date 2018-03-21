extern crate rand;
#[macro_use]
extern crate rand_derive;
// #[macro_use]
// extern crate lazy_static;
extern crate colored;
extern crate indicatif;

// mod translator;
mod parser;
mod generator;
mod node;
mod map;
mod solver;
mod solved;

pub use std::collections::{HashMap, BinaryHeap};
pub use map::{Map,Point,Heuristic};
pub use node::Node;
// use rand::Rng;
// use std::cmp::Ordering;
use colored::*;
pub use solver::Solver;
pub use solved::Solved;
pub use indicatif::{ProgressBar, ProgressStyle};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Rand, Hash)]
pub enum Movement {
    Up,
    Down,
    Left,
    Right,
    No,
}

pub fn create_progress_bar() -> ProgressBar {
    let bar = ProgressBar::new_spinner();
    bar.set_message("Solving");
    bar.set_style(ProgressStyle::default_bar()
                  .template("Solving {spinner:.green} [{elapsed_precise}] | Closeset size - {len:.green} | Openset size - {msg:.green} | Current H - {pos:.red}"));
    bar
}

pub fn process(mut start_node: Node, extra: bool) -> Result<Solved, &'static str> {
    let mut closeset = Vec::<Box<Node>>::new();
    let mut openset  = BinaryHeap::<Box<Node>>::new();
    let mut hashmap: HashMap<Vec<u16>, u16> = HashMap::new();
    let mut complextity: usize = 0;
    let mut memory: usize = 0;
    let un_translated_node = start_node.clone();
    let h: u16;

    if let Some(ref mut map) = start_node.map {
        println!("Starting from :\n");
        map.display();
        map.translate_in();
        map.check_validity()?;
        map.set_first_costs();
        h = map.get_cost();
    } else {
        return Err("Weird problem going on...");
    }

    start_node.h = h;
    start_node.f = h;
    openset.push(Box::new(start_node));
    let bar = create_progress_bar();
    loop {
        if let Some(last) = closeset.last() {
            if (*last).h == 0 {
                break;
            }
        }
        let mut node = openset.pop().unwrap();
        if extra {
            println!("Current node:\n\tH: {:?}, G: {:?}, F: {:?}\nOther infos:\n\tOpenset size: {:?}\n\tCloseset size: {:?}", node.h, node.g, node.f, openset.len(), closeset.len());
        } else {
            bar.set_length(closeset.len() as u64);
            bar.set_message(format!("{:?}", openset.len()).as_str());
            bar.set_position(node.h as u64);
        }
        let index = closeset.len();
        let childs = node.get_childs(index, &mut hashmap);
        closeset.push(node);
        for child in childs.into_iter() {
            openset.push(child);
            complextity += 1;
        }
        if openset.len() + closeset.len() + hashmap.len() > memory {
            memory = openset.len() + closeset.len() + hashmap.len();
        }
    }
    bar.finish_and_clear();
    let mut sequence = Vec::<Movement>::new();
    let end = closeset.pop().unwrap();
    sequence.push((*end).movement);
    let mut index = (*end).parent;
    while index != 0 {
        let node = closeset.remove(index);
        index = (*node).parent;
        sequence.push((*node).movement);
    }
    sequence.reverse();
    //println!("{:#?}", openset);
    Ok(Solved {
        memory: memory,
        complexity: complextity,
        sequence: sequence,
        start_node: un_translated_node,
    })
}

pub fn parse(filename: &str, func: Heuristic, boost: u16, greedy: bool) -> Result<Node, &'static str> {
    let (vec_spiral, point, size) = match parser::parse(filename) {
        Ok(x) => x,
        Err(msg) => {println!("{}", msg.red()); return Err("Failed to parse")},
    };
    let solver = Solver::new(size, func, boost, greedy);
    let map = Map::new(vec_spiral, &solver, point, None);
    Ok(Node::new_from_map(map))
}

pub fn create_random(size: u16, func: Heuristic, boost: u16, greedy: bool) -> Result<Node, &'static str> {
    let solver: &Solver = Solver::new(size, func, boost, greedy);
    let zero_index = solver.zero_index;
    let mut vec_spiral = generator::create_solved_spiral(size as i16); //TODO remove this generation and clone solver
    vec_spiral[zero_index as usize] = 0;

    let mut map = Map::new(vec_spiral, &solver, Point{x: zero_index % size, y: zero_index / size}, None);
    map.shuffle();
    Ok(Node::new_from_map(map))
}
