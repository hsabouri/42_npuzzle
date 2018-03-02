extern crate rand;
use rand::Rng;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Movement {
    Up,
    Down,
    Left,
    Right,
    No,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Heuristic {
    Manhattan,
    Wrong,
    Linear,
    Composit,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Map {
    pub content: Vec<usize>,
    pub pos: Point,
    pub size: usize,
    pub costs: Vec<usize>,
}

fn h_wrong(map: &Map, old: Option<&Map>, solved: &Map) -> Vec<usize> {
    match old {
        Some(_) => {
            let unwrappedOld = old.unwrap();
            let pos = map.pos.x + map.pos.y * map.size;
            let mut res = unwrappedOld.costs.clone();

            res[pos] = if res[pos] == solved.content[pos] {0} else {2};
            res
        },
        None => {
            let mut res = Vec::<usize>::new();

            for (i, value) in map.content.iter().enumerate() {
                res.push(if *value == solved.content[i] {0} else {2});
            }
            res
        }
    }
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

    pub fn get_costs(&self, old: Option<&Map>, solved: &Map, func: Heuristic) -> Vec<usize> {
        match func {
            _ => h_wrong(self, old, solved)
        }
    }

    pub fn get_cost(&self, old: Option<&Map>, solved: &Map) -> usize {
        self.get_costs(old, solved, Heuristic::Wrong).iter().fold(0, |acc, &x| acc + x)
    }
    
    pub fn child(&mut self, movement: &Movement) {
        self.content.swap(self.pos.x + self.pos.y * self.size, {
            match *movement {
                Movement::Down => self.pos.x + (self.pos.y - 1) * self.size,
                Movement::Up => self.pos.x + (self.pos.y + 1) * self.size,
                Movement::Right => (self.pos.x - 1) + self.pos.y * self.size,
                Movement::Left => (self.pos.x + 1) + self.pos.y * self.size,
                Movement::No => self.pos.x + self.pos.y * self.size,
            }
        });

        self.pos = match *movement {
            Movement::Right => Point {x: self.pos.x - 1, y: self.pos.y},
            Movement::Left => Point {x: self.pos.x + 1, y: self.pos.y},
            Movement::Down => Point {x: self.pos.x, y: self.pos.y - 1},
            Movement::Up => Point {x: self.pos.x, y: self.pos.y + 1},
            Movement::No => Point {x: self.pos.x, y: self.pos.y},
        };
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
            pos: Point {
                x: match side % 2 {
                    0 => side / 2 - 1,
                    _ => side / 2,
                },
                y: side / 2
            },
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

    pub fn gen(size: usize, solved: &Map) -> Map {
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
        let mut res = Map {
            content: content,
            pos: pos,
            size: size,
            costs: (0..(size * size)).collect(),
        };
        res.costs = res.get_costs(None, solved, Heuristic::Wrong);
        res
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Node {
    pub map: Option<Map>,
    pub parent: usize,
    pub movement: Movement,
    pub hash: usize, // Can basically be a weighted addition of the content and the complexity
    pub g: usize,
    pub h: usize,
    pub f: usize,
}

impl Node {
    pub fn new(map: Map, parent: usize, movement: Movement, hash: usize, g: usize, h: usize, f: usize) -> Node {
        Node {
            map: Some(map),
            parent: parent,
            movement: movement,
            hash: hash,
            g: g,
            h: h,
            f: f,
        }
    }

    pub fn child(&mut self, movement: Movement, parent: usize, solved: &Map) -> Node {
        let mut map = self.map.clone().unwrap();

        map.child(&movement);
        let h = map.get_cost(None, &solved);
        Node {
            map: Some(map),
            parent: parent,
            movement: movement,
            hash: 0, //TODO
            g: self.g + 1,
            h: h,
            f: self.g + 1 + h,
        }
    }

    pub fn gen(size: usize) -> (Node, Node) {
        let solved = Map::get_solved(size);
        let map = Map::gen(size, &solved);
        let h = map.get_cost(None, &solved);
            
        (Node {
            map: Some(map),
            parent: 0,
            movement: Movement::No,
            hash: 0,
            g: 0,
            h: h,
            f: h,
        }, Node {
             map: Some(solved),
            parent: 0,
            movement: Movement::No,
            hash: 0,
            g: 0,
            h: 0,
            f: 0,

        })
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        self.f.cmp(&other.f)
    }
}
