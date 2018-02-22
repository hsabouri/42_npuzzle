extern crate rand;
use rand::Rng;

#[derive(Debug, Clone)]
pub enum Movement {
    Up,
    Down,
    Left,
    Right,
    No,
}

#[derive(Debug, Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone)]
pub struct Map {
    content: Vec<usize>,
    pos: Point,
    size: usize,
    costs: Vec<usize>,
}

impl Map {
    pub fn new(content: Vec<usize>, pos: Point, size: usize, costs: Vec<usize>) -> Map {
        Map {
            content: content,
            pos: pos,
            size: size,
            costs: costs,
        }
    }
    
    pub fn child(old: &Map, movement: &Movement) -> Map {
        let mut res = old.clone();

        res.content.swap(old.pos.x + old.pos.y * old.size, {
            match *movement {
                Movement::Down => old.pos.x + (old.pos.y - 1) * old.size,
                Movement::Up => old.pos.x + (old.pos.y + 1) * old.size,
                Movement::Right => (old.pos.x - 1) + old.pos.y * old.size,
                Movement::Left => (old.pos.x + 1) + old.pos.y * old.size,
                Movement::No => old.pos.x + old.pos.y * old.size,
            }
        });

        res.pos = match *movement {
            Movement::Right => Point {x: old.pos.x - 1, y: old.pos.y},
            Movement::Left => Point {x: old.pos.x + 1, y: old.pos.y},
            Movement::Down => Point {x: old.pos.x, y: old.pos.y - 1},
            Movement::Up => Point {x: old.pos.x, y: old.pos.y + 1},
            Movement::No => Point {x: old.pos.x, y: old.pos.y},
        };
        res
    }

    pub fn gen(size: usize) -> Map {
        let mut topush: Vec<usize> = (0..(size * size)).collect();
        let mut pos = Point {x: 0, y: 0};
        let content: Vec<usize> = (0..(size * size)).map(|map_id: usize| {
            let id = rand::thread_rng().gen_range(0, topush.len());
            let res = topush[id];

            topush.remove(id);
            if res == 0 {
                pos = Point {x: map_id % size, y: map_id / size};
            }
            res
        }).collect();
        Map {
            content: content,
            pos: pos,
            size: size,
            costs: (0..(size * size)).collect(),
        }
    }

    pub fn get_solved(&self) -> Map {
        
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    map: Map,
    parent: usize,
    movement: Movement,
    hash: usize, // Can basically be a weighted addition of the content and the complexity
    g: usize,
    h: usize,
    f: usize,
}

impl Node {
    pub fn new(map: Map, parent: usize, movement: Movement, hash: usize, g: usize, h: usize, f: usize) -> Node {
        Node {
            map: map,
            parent: parent,
            movement: movement,
            hash: hash,
            g: g,
            h: h,
            f: f,
        }
    }

    pub fn child(old: &Node, movement: Movement, parent: usize) -> Node {
        let random: usize = rand::random::<usize>();
        Node {
            map: Map::child(&old.map, &movement),
            parent: parent,
            movement: movement,
            hash: 0, // TODO
            g: old.g + 1,
            h: random, // TODO
            f: old.g + 1 + random, // TODO
        }
    }

    pub fn gen(size: usize) -> (Node, Node) {
        let random: usize = rand::random::<usize>();
        Node {
            map: Map::gen(size),
            parent: 0,
            movement: Movement::No,
            hash: 0, //TODO
            g: 0,
            h: random, //TODO
            f: random, //TODO
        }
    }
}
