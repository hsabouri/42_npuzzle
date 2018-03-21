#[macro_use]
extern crate clap;
extern crate lib_npuzzle;
extern crate colored;

use lib_npuzzle::{Node, Heuristic, Solved};
use colored::*;
use clap::{Arg, App, ArgMatches};

fn display(solved: Solved, verbose: bool) {
    let mut map = solved.start_node.map.unwrap();
    for movement in solved.sequence.iter() {
        if verbose {
            map.do_move(movement);
            println!("{:?}:", movement);
            map.display();
        } else {
            println!("{:?}", movement);
        }
    }
    println!("Maximum states represented in memory : {:?}", solved.memory);
    println!("States selected for the openset : {:?}", solved.complexity);
    println!("Solution is made of {:?} moves", solved.sequence.len());
}

fn init_map(matches: &ArgMatches) -> Result<Node, &'static str> {
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
    let boost = match matches.value_of("boost") {
        Some(value) => {
            let parsed = value.parse::<u16>().unwrap_or(1);
            if parsed > 0 { parsed } else {1}
        },
        None => 1
    };
    match matches.value_of("FILE") {
        Some(filename) => lib_npuzzle::parse(filename, heuristic_func, boost),
        None => {
            let size = value_t!(matches.value_of("SIZE"), u16).unwrap_or_else(|e| e.exit());
            match size {
                size if size > 20   => Err("Ah ah nice try. it's too big !."),
                size if size < 3    => Err("Size must be equals or higher than 3."),
                size                => lib_npuzzle::create_random(size, heuristic_func, boost),
            }
        }
    }
}

fn do_the_job(matches: ArgMatches) -> Result<(), &'static str> {
    let start_node = match init_map(&matches) {
        Ok(x)       => x,
        Err(msg)    => {println!("{}", msg.red()); return Err("Failed to init map")}
    };
    let (verbose, extra) = match matches.occurrences_of("v") {
        0 => (false, false),
        1 => (true, false),
        _ => (true, true),
    };
    let result = lib_npuzzle::process(start_node, extra)?;
    display(result, verbose);
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
            .help("Size of a random generated N-puzzle [2 - 20]"))
        .arg(Arg::with_name("H")
            .help("Heuristic chosen to solve the puzzle")
            .short("H")
            .takes_value(true)
            .long("heuristic"))
        .arg(Arg::with_name("boost")
            .help("Boost multiplier")
            .short("b")
            .takes_value(true)
            .long("boost"))
        .arg(Arg::with_name("v")
            .help("Sets the level of verbosity")
            .multiple(true)
            .short("v"))
        .get_matches();

    if let Err(msg) = do_the_job(matches) {
        println!("{}", msg.red());
    }
    //TODO display result depending of verbosity ?
}
