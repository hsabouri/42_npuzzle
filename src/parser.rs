use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

use lib;
use error;

pub fn parse(filename: &str) -> (lib::Node, lib::Node) {
    match File::open(filename) {
        Ok(v) => {
            let mut buff = BufReader::new(&v);
            for line in buff.lines() {
                let l = line.unwrap();
                println!("{}", l); 
            }
        },
        Err(e) => error::exit("Could not open file"),
    };
    lib::Node::gen(3)
}
