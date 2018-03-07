extern crate rand;

use super::{from_index_to_value,Point};

fn spiral(w: i16, h: i16, x: i16, y: i16) -> u16 {
    if y == 0 {
        (x + 1) as u16
    } else {
        w as u16 + spiral(h - 1, w, y - 1, w - x - 1)
    }
}
    pub fn get_solved(size: u16) -> (Vec<u16>, Point) {
        let mut map: Vec<u16> = Vec::new();
        let square_size = size * size;
        let mut pos: Point = Point {x: 0, y: 0};
        for x in 0..size {
            for y in 0..size {
                map.push(match spiral(size as i16, size as i16, y as i16, x as i16) {
                    var if var == square_size => {pos = Point {x: x, y:y}; 0},
                    var     => var
                });
            }
        }
        (map, pos)
    }

fn create_solved_spiral(size: i16) -> Vec<u16> {
    let mut map: Vec<u16> = Vec::new();
    let zero_pos = match size % 2 {
        0 => size / 2 - 1 + (size / 2) * size,
        _ => size / 2 + (size / 2) * size,
    };
    for x in 0..size {
        for y in 0..size {
            map.push(spiral(size, size, y, x));
        }
    }
    map[zero_pos as usize] = 0;
    map
}

static mut SPIRAL: Spiral = Spiral {
    content: None,
    size: 0,
    sq_size: 0
};

pub struct Spiral {
    content: Option<Vec<u16>>,
    size: usize,
    sq_size: usize
}

impl Spiral {
    pub fn init(size: usize) {
        unsafe {SPIRAL = Spiral {
            content: Some(create_solved_spiral(size as i16)),
            size: size,
            sq_size: size * size
        };}
    }

    fn value_to_index(&self, value: u16) -> u16 {
        match self.content {
            None => panic!("please init first"),
            Some(ref val) =>
            for i in 0..self.sq_size {
                if value == val[i] {
                    return i as u16;
                }
            }
        }
        panic!("There is no {} in solved", value);
    }

    pub fn translate_in(&self, input: Vec<u16>) -> Vec<u16> {
        let mut map: Vec<u16> = vec![0; self.sq_size as usize];
        for i in 0..self.sq_size {
            map[i] = super::from_index_to_value(self.value_to_index(input[i]));
        }
        map

    }
}

pub fn translate_in(input: Vec<u16>) -> Vec<u16> {
    unsafe{SPIRAL.translate_in(input)}
}
