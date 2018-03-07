#[macro_use]
extern crate clap;
extern crate lib_npuzzle;
extern crate colored;

use lib_npuzzle::Node;
use colored::*;
use clap::{Arg, App, ArgMatches};

fn init_map(matches: ArgMatches) -> Result<(Node, Node), &'static str> {
    match matches.value_of("FILE") {
        Some(filename) => lib_npuzzle::parse(filename),
        None => {
            let size = value_t!(matches.value_of("SIZE"), u16).unwrap_or_else(|e| e.exit());
            match size {
                size if size > 20   => Err("Ah ah nice try. it's too big !."),
                size if size < 3    => Err("Size must be equals or higher than 3."),
                size                => lib_npuzzle::create_random(size),
            }
        }
    }
}

fn main() {
    let matches = App::new("npuzzle")
        .version("1.0")
        .about("Solves n-puzzles")
        .author("Hugo Sabourin <hsabouri@student.42.fr>, William Escande <wescande@student.42.fr>")
        .arg(Arg::with_name("FILE")
            .help("Input file containing puzzle to solve")
            .index(1))
        .arg(Arg::with_name("SIZE").short("s").long("size").default_value("3").takes_value(true).help("Size of a random generated N-puzzle [3 - 20]"))
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
        Err(msg)    => println!("{}{}", "Failed to init map: ".red(), msg.red()),
        Ok((map, solved))     => lib_npuzzle::solve(map, solved)
    };
    //TODO display result depending of verbosity ?
}
