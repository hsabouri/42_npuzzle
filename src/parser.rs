use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

use lib;
use error;

pub fn parse(filename: &str) -> Result <lib::Map, String> {
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
                    Err(_) => return Err(format!("File is not valid\n  '{}' is not an unsigned number", to_parse)),
                    Ok(n) => n,
                }
            },
            n => return Err(format!("File is not valid\n  in '{}'\n  Expected only one token, got {}", line.join(" "), n)),
        }
    };

    if size < 3 {
        return Err(format!("Puzzle is not valid\n  Minimum puzzle size is 3, got {}.", size));
    }
    if lines.len() != size {
        return Err(format!("File is not valid\n  Expected {} lines to describe puzzle, had {}.", size, lines.len()));
    }
    
    let mut pos: lib::Point = lib::Point {x: 0, y: 0};
    let mut costs: Vec<Option<usize>> = (0..(size * size)).map(|x| None).collect();
        // Also used to check number's occurences
    let map = lib::Map::new({
        let mut res = Vec::<usize>::new();

        for (y, line) in lines.iter().enumerate() {
            if line.len() != size {
                return Err(format!("Puzzle is not valid\n  in '{}'\n  Expected {} tokens, got {}", line.join(" "), size, line.len()));
            }
            for (x, token) in line.iter().enumerate() {
                let parsed = token.parse::<usize>();
                let n = match parsed {
                    Err(_) => return Err(format!("File is not valid\n  '{}' is not an unsigned number", token)),
                    Ok(n) => n,
                };
                if n >= size * size {
                    return Err(format!("Puzzle is not valid\n  Expected a number under {}, got {}", size * size, token));
                }
                if costs[n].is_some() {
                    return Err(format!("Puzzle is not valid\n  {} already exists", n));
                } else {
                    costs[n] = Some(0);
                }
                if n == 0 {
                    pos = lib::Point {x: x, y: y};
                }
                res.push(n);
            }
        }
        res
    }, pos, size, costs.iter().map(|x| x.unwrap()).collect());
    Ok(map)
}
