extern crate rand;

use super::Point;

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

// TODO
// pub fn translate_in() {

// }

// pub fn translate_out() {

// }
