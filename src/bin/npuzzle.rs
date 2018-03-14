#[macro_use]
extern crate clap;
extern crate lib_npuzzle;
extern crate colored;

use lib_npuzzle::{Node, Heuristic};
use colored::*;
use clap::{Arg, App, ArgMatches};

fn init_map(matches: ArgMatches) -> Result<Node, &'static str> {
    let heuristic_func = match matches.value_of("H") {
        Some(func) => {
            match func {
                "manhattan" | "m" => Heuristic::Manhattan,
                "naive" | "n" => Heuristic::Naive,
                "linear" | "l" => Heuristic::Linear,
                _ => return Err("Heuristic does not exist"),
            }
        },
        None => Heuristic::Manhattan,
    };
    match matches.value_of("FILE") {
        Some(filename) => lib_npuzzle::parse(filename, heuristic_func),
        None => {
            let size = value_t!(matches.value_of("SIZE"), u16).unwrap_or_else(|e| e.exit());
            match size {
                size if size > 20   => Err("Ah ah nice try. it's too big !."),
                size if size < 3    => Err("Size must be equals or higher than 3."),
                size                => lib_npuzzle::create_random(size, heuristic_func),
            }
        }
    }
}
fn do_the_job(matches: ArgMatches) -> Result<(), &'static str> {
    let start_node = match init_map(matches) {
        Ok(x)       => x,
        Err(msg)    => {println!("{}", msg.red()); return Err("Failed to init map")}
    };
    lib_npuzzle::process(start_node)?;
    Ok(())
}

fn main() {
    let matches = App::new("npuzzle")
        .version("1.0")
        .about("Solves n-puzzles")
        .author("Hugo Sabourin <hsabouri@student.42.fr>, William Escande <wescande@student.42.fr>")
        .arg(Arg::with_name("FILE")
            .help("Input file containing puzzle to solve")
            .index(1))
        .arg(Arg::with_name("SIZE")
            .short("s")
            .long("size")
            .default_value("3")
            .takes_value(true)
            .help("Size of a random generated N-puzzle [3 - 20]"))
        .arg(Arg::with_name("H")
            .help("Heuristic chosen to solve the puzzle")
            .short("H")
            .takes_value(true)
            .long("heuristic"))
        .arg(Arg::with_name("v")
            .help("Sets the level verbosity")
            .short("v")
            .multiple(true))
        .get_matches();

    if let Err(msg) = do_the_job(matches) {
        println!("{}", msg.red());
    }
    //TODO display result depending of verbosity ?
}
