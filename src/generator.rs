fn spiral(w: i16, h: i16, x: i16, y: i16) -> u16 {
    match y {
        0 => (x + 1) as u16,
        y => w as u16 + spiral(h - 1, w, y - 1, w - x - 1)
    }
}

pub fn create_solved_spiral(size: i16) -> Vec<u16> {
    let mut map: Vec<u16> = Vec::new();
    for x in 0..size {
        for y in 0..size {
            map.push(spiral(size, size, y, x));
        }
    }
    map
}
