use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

use lib;
use error;

pub fn parse(filename: &str) -> (lib::Node, lib::Node) {
    match File::open(filename) {
        Ok(v) => {
            let buff = BufReader::new(&v);
            let lines: Vec<String> = buff.lines().map(|line| line.unwrap()).collect();
        },
        Err(e) => error::exit("Could not open file"),
    };
    lib::Node::gen(3)
}
