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

    pub fn get_solved(side: usize) -> Map {
        let (mut x, mut y) = (
            match side % 2 {
                0 => side / 2,
                _ => side / 2 - 1,
            },
            match side % 2 {
                0 => side / 2 + 1,
                _ => side / 2 - 1,
            }
        );
        let size = side * side;
        let mut map: Vec<usize> = (0..size).map(|_| 0).collect();
        let mut direction: Movement = match side % 2 {
            0 => Movement::Up,
            _ => Movement::Down,
        };
        let mut n = size - 1;

        for turn in 0..(side * 2 - 2) {
            let to_push: usize = {
                if turn == 0 || turn == 1 {
                    2
                } else if turn == (side * 2 - 3)  {
                    side - 1 
                } else {
                    (turn + 1) / 2 + 1
                }
            };
            println!("turn: {}, to_push {}, direction: {:?}", turn, to_push, direction);
            for _ in 0..to_push {
                x = match direction {
                    Movement::Left => x - 1,
                    Movement::Right => x + 1,
                    _ => x,
                };
                y = match direction {
                    Movement::Up => y - 1,
                    Movement::Down=> y + 1,
                    _ => y,
                };
                println!("n: {}, x: {}, y: {}", n, x, y);
                map[x + y * side] = n;
                n = n - 1;
            }
            direction = match direction {
                Movement::Up => Movement::Left,
                Movement::Left => Movement::Down,
                Movement::Down => Movement::Right,
                Movement::Right => Movement::Up,
                _ => Movement::No,
            };
        }
        Map {
            content: map,
            pos: Point {x: side / 2, y: side / 2},
            size: side,
            costs: (0..(size - 1)).map(|_| 0).collect(),
        }
    }

    pub fn display(&self) {
        for y in 0..self.size {
            let mut to_display = String::from("");
            for x in 0..self.size {
                to_display.push_str(format!("{:4}", self.content[x + y * self.size]).as_str());
            }
            println!("{}\n", to_display);
        }
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
}

#[derive(Debug, Clone)]
pub struct Node {
    pub map: Map,
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
        (Node {
            map: Map::gen(size),
            parent: 0,
            movement: Movement::No,
            hash: 0, //TODO
            g: 0,
            h: random, //TODO
            f: random, //TODO
        }, Node {
            map: Map::get_solved(size),
            parent: 0,
            movement: Movement::No,
            hash: 0, //TODO
            g: 0,
            h: 0,
            f: 0,
        })
    }
}
