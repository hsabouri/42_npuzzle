extern crate clap;
use clap::{Arg, App};

extern crate rand;
mod lib;
mod error;
mod parser;

fn get_childs(node: &mut lib::Node, solved: &lib::Map, index: usize) -> Vec<lib::Node> {
    let map = node.map.take().unwrap(); 
    let mut to_explore = Vec::<lib::Movement>::new();

    if map.pos.x < map.size - 1 && node.movement != lib::Movement::Right {
        to_explore.push(lib::Movement::Left);
    }
    if map.pos.x > 0 && node.movement != lib::Movement::Left {
        to_explore.push(lib::Movement::Right); 
    }
    if map.pos.y < map.size - 1 && node.movement != lib::Movement::Down {
        to_explore.push(lib::Movement::Up);
    }
    if map.pos.y > 0 && node.movement != lib::Movement::Up {
        to_explore.push(lib::Movement::Down); 
    }

    node.map = Some(map);
    to_explore.iter().map(|dir| node.child(*dir, index, solved)).collect()
}

fn push_sorted(mut list: Vec<lib::Node>, mut to_push: Vec<lib::Node>) -> Vec<lib::Node> {
    if list.len() == 0 {
        list.push(to_push.pop().unwrap());
    }

    let len = to_push.len();

    for _ in 0..len {
        let node = to_push.pop().unwrap();
        let index = list.binary_search(&node).unwrap_or_else(|e| e);

        list.insert(index, node);
    }
    list
}

fn main() {
    let matches = App::new("npuzzle")
        .version("1.0")
        .about("Solves n-puzzles")
        .author("hsabouri")
        .arg(Arg::with_name("FILE")
            .help("Input file containing puzzle to solve")
            .index(1))
        .arg(Arg::with_name("size")
            .help("Define size of input puzzle")
            .short("s")
            .long("size"))
        .arg(Arg::with_name("H")
            .help("Heuristic chosen to solve the puzzle")
            .short("H")
            .long("heuristic"))
        .arg(Arg::with_name("v")
            .help("Sets the level verbosity")
            .short("v")
            .multiple(true))
        .get_matches();

    let filename = matches.value_of("FILE");

    let res = match filename {
        None => {
            let size = matches.value_of("size").unwrap_or("3").parse::<usize>().unwrap();

            if size < 3 {
                error::exit("Size must be equals or higher than 3.");
            }
            Ok(lib::Node::gen(size))
        },
        Some(_) => parser::parse(filename.unwrap())
    };

    match res {
        Err(e) => println!("\x1b[31mError:\x1b[0m {}", e),
        Ok(v) => {}
    }
    /*
    let nodes = lib::Node::gen(3);
    let solved = nodes.1.map.unwrap();
    let mut node = nodes.0;
    let mut openl = Vec::<lib::Node>::new();
    let mut closel = Vec::<lib::Node>::new();

    let childs = get_childs(&mut node, &solved, 0);

    closel.push(node);
    openl = push_sorted(openl, childs);
    while closel.last().unwrap().h > 0 {
        node = openl.remove(0);
        let childs = get_childs(&mut node, &solved, closel.len());

        closel.push(node);
        openl = push_sorted(openl, childs);
        println!("{:4} - {:4} - {:4}", closel.last().unwrap().f, closel.last().unwrap().h, closel.last().unwrap().g);
    }
    let mut i = closel.len();
    let mut last = closel.pop().unwrap();
    while i > 0 {
        let last_map = last.map.take().unwrap();

        last_map.display();
        println!("\n");
        i = last.parent;
        last = closel.remove(i);
    }
    */
}
