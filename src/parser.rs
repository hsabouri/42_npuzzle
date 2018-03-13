use super::Point;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn file_to_str_array(filename: &str) -> Result <Vec<Vec<String>>, String> {
    let file = match File::open(filename) {
        Ok(n) => n,
        Err(e) => return Err(format!("Could not open file '{}':\n{}", filename, e)),
    };
    let buff = BufReader::new(&file);
    let lines: Vec<Vec<String>> = buff.lines().filter_map( |line| { // REMOVING 
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
    Ok(lines)
}

fn get_map_size(line: Vec<String>) -> Result <u16, String> {
    match line.len() {
        1 => {
            match line[0].parse::<u16>() {
                Err(msg) => return Err(format!("File is not valid\n  '{}' is not an unsigned number\n>'{}'", line[0], msg)),
                Ok(n) if n < 3 => return Err(format!("Puzzle is not valid\n  Minimum puzzle size is 3, got {}.", n)),
                Ok(n) if n > 20 => return Err(format!("Puzzle is not valid\n  Maximum puzzle size is 20, got {}.", n)),
                Ok(n) => Ok(n),
            }
        },
        n => return Err(format!("File is not valid\n  in '{}'\n  Expected only one token, got {}", line.join(" "), n)),
    }
}

pub fn parse(filename: &str) -> Result <(Vec<u16>, Point, u16), String> {
    let mut lines = file_to_str_array(filename)?;
    let size = get_map_size(lines.remove(0))?;


    if lines.len() != size as usize{
        return Err(format!("File is not valid\n  Expected {} lines to describe puzzle, had {}.", size, lines.len()));
    }

    let mut pos: Point = Point {x: 0, y: 0};
    let mut res = Vec::<u16>::new();

    for (y, line) in lines.iter().enumerate() {
        if line.len() != size as usize {
            return Err(format!("Puzzle is not valid\n  in '{}'\n  Expected {} tokens, got {}", line.join(" "), size, line.len()));
        }
        for (x, token) in line.iter().enumerate() {
            let number = match token.parse::<u16>(){
                Err(_) => return Err(format!("File is not valid\n  '{}' is not an unsigned number", token)),
                Ok(number) => number,
            };
            if let Some(_) = res.iter().find(|&&x| x == number) {
                return Err(format!("Puzzle is not valid\n  Number {} already present", number));
            }
            if number >= size * size {
                return Err(format!("Puzzle is not valid\n  Expected a number under {}, got {}", size * size, token));
            }
            if number == 0 {
                pos = Point {x: x as u16, y: y as u16};
            }
            res.push(number);
        }
    }
    Ok((res, pos, size))
}
