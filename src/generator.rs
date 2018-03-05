extern crate rand;

use rand::Rng;
use super::Point;

fn spiral(w: i16, h: i16, x: i16, y: i16) -> u16 {
    if y == 0 {
        (x + 1) as u16
    } else {
        w as u16 + spiral(h - 1, w, y - 1, w - x - 1)
    }
}

// pub fn get_random(size: i16) -> Vec<i16> {
//     let solved = get_solved(size);
//     let pos = Pointi {
//         x: match size % 2 {
//             0 => size / 2 - 1,
//             _ => size / 2,
//         },
//         y: size / 2
//     };
//     let iter = rand::thread_rng().gen_range(50, 150);
//     for _ in 0..iter {
//     }
//     solved
// }

pub fn get_solved(size: u16) -> (Vec<u16>, Point) {
    let mut map: Vec<u16> = Vec::new();
    let square_size = size * size;
    let mut pos: Point;
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
