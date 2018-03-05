use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

use lib;
use error;

pub fn parse(filename: &str) -> Result <(lib::Node, lib::Node), String> {
    let file = match File::open(filename) {
        Ok(n) => n,
        Err(e) => return Err(format!("Could not open file '{}'", filename)),
    };
    let buff = BufReader::new(&file);
    let mut lines: Vec<Vec<String>> = buff.lines().filter_map( |line| { // REMOVING 
        let unwrapped = line.unwrap();
        match unwrapped.find("#") { // CHECKING COMMENTS
            Some(0) => None, // Line is a comment
            Some(_) => { // Line contains a comment
                let uncommented: Vec<String> = unwrapped.split("#")
                                                        .next()
                                                        .unwrap()
                                                        .split_whitespace()
                                                        .map(|s| String::from(s))
                                                        .collect();
                if uncommented.len() > 0 { Some(uncommented) } else { None }
            },
            None => { // No comment in line
                let uncommented: Vec<String>= unwrapped.split_whitespace()
                                                       .map(|s| String::from(s))
                                                       .collect();
                if uncommented.len() > 0 { Some(uncommented) } else { None }
            }
        }
    }).collect();

    if lines.is_empty() {
        return Err(format!("File is empty or contains only comments/new-lines."));
    }
    
    let size = {
        let mut line = lines.remove(0);

        match line.len() {
            1 => {
                let to_parse = line.remove(0);

                match to_parse.parse::<usize>() {
                    Err(_) => return Err(format!("File is not valid\n  '{}' is not a number", to_parse)),
                    Ok(n) => n,
                }
            },
            n => return Err(format!("File is not valid\n  '{}'\n  Expected only one token, got {}", line.join(" "), n))
        }
    };
    println!("Map size : {}", size);
    Ok(lib::Node::gen(3)) 
}
