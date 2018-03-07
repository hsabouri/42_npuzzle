extern crate clap;
extern crate lib_npuzzle;
extern crate colored;

use lib_npuzzle::Node;
use colored::*;
use clap::{Arg, App, ArgMatches};

fn init_map(matches: ArgMatches) -> Result<(Node, Node), &'static str> {
    match matches.value_of("FILE") {
        Some(filename) => Ok(lib_npuzzle::parse(filename)?),
        None => {
            let size = matches.value_of("SIZE").unwrap_or("3").parse::<u16>().unwrap(); // TODO not an unwrap
            if size > 20 {
                Err("Ah ah nice try. it's too big !.")
            } else if size < 3 {
                Err("Size must be equals or higher than 3.")
            } else {
                Ok(lib_npuzzle::create_random(size)?)
            }
        }
    }
}

fn main() {
    let matches = App::new("npuzzle")
        .version("1.0")
        .about("Solves n-puzzles")
        .author("hsabouri")
        .arg(Arg::with_name("FILE")
            .help("Input file containing puzzle to solve")
            .index(1))
        .arg(Arg::with_name("SIZE")
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

    match init_map(matches) {
        Err(msg)    => println!("Failed to init map: {}", msg.red()),
        Ok((map, solved))     => lib_npuzzle::solve(map, solved)
    };
}
