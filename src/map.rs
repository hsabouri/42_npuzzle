extern crate rand;

use rand::Rng;
use super::Movement;
use solver::Solver;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Map {
    pub content: Vec<u16>,
    pub solver: &'static Solver,
    pub pos: Point,
    pub costs: Option<Vec<usize>>,
}

impl Map {
    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        for _ in 0..100000 {
            let random_move: Movement = rng.gen();
            if random_move != Movement::No {
                if self.can_move(random_move) {
                    self.do_move(random_move);
                }
            }
        }
    }

    pub fn translate_in(&mut self) {
        self.content = self.solver.translate_in(&self.content);
    }

    pub fn new(content: Vec<u16>, ref solver: &'static Solver, pos: Point, costs: Option<Vec<usize>>) -> Map {
        Map {content: content, solver: solver, pos: pos, costs: costs}
    }

    fn can_move(&self, direction: Movement) -> bool {
        match direction {
            Movement::Up => self.pos.y > 0,
            Movement::Down => self.pos.y < (self.solver.size - 1),
            Movement::Left => self.pos.x > 0,
            Movement::Right => self.pos.x < (self.solver.size - 1),
            Movement::No => true,
        }
    }

    fn do_move(&mut self, direction: Movement) {
        self.content.swap((self.pos.x + self.pos.y * self.solver.size) as usize,
        (match direction {
            Movement::Up => self.pos.x + (self.pos.y - 1) * self.solver.size,
            Movement::Down => self.pos.x + (self.pos.y + 1) * self.solver.size,
            Movement::Left => (self.pos.x - 1) + self.pos.y * self.solver.size,
            Movement::Right => (self.pos.x + 1) + self.pos.y * self.solver.size,
            Movement::No => self.pos.x + self.pos.y * self.solver.size
        }) as usize
        );

        self.pos = match direction {
            Movement::Up => Point {x: self.pos.x, y: self.pos.y - 1},
            Movement::Down => Point {x: self.pos.x, y: self.pos.y + 1},
            Movement::Left => Point {x: self.pos.x - 1, y: self.pos.y},
            Movement::Right => Point {x: self.pos.x + 1, y: self.pos.y},
            Movement::No => Point {x: self.pos.x, y: self.pos.y},
        };
    }

    // fn first_heuristic_naive(&self) -> Vec<u16> {
    //     let mut res = Vec::<u16>::new();

    //     for (index, value) in self.content.iter().enumerate() {
    //         let solved_value = from_index_to_value(index as u16);

    //         if solved_value == *value {
    //             res.push(0);
    //         } else {
    //             res.push(10);
    //         }
    //     }
    //     res
    // }

    //pub fn first_get_costs(&self, func: Heuristic) -> Vec<u16> {
    //    match func {
    //        //Heuristic::Linear => self.heuristic_linear(solved),
    //        Heuristic::Naive => self.first_heuristic_naive(),
    //        _ => self.first_heuristic_naive(),
    //        //_ => self.heuristic_manhattan(solved),
    //    }
    //}

    // pub fn get_costs(&self, old: Option<&Map>, solved: &Map, func: Heuristic) -> Vec<u16> {
    //     match func {
    //         _ => h_wrong(self, old, solved)
    //     }
    // }

    // pub fn get_cost(&self, old: Option<&Map>, solved: &Map) -> usize {
    //     self.get_costs(old, solved, Heuristic::Wrong).iter().fold(0, |acc, &x| acc + x as usize)
    // }

    // pub fn child(&mut self, movement: &Movement) {
    //     self.content.swap(self.pos.x + self.pos.y * unsafe {SOLVER.size}, {
    //         match *movement {
    //             Movement::Down => self.pos.x + (self.pos.y - 1) * unsafe {SOLVER.size},
    //             Movement::Up => self.pos.x + (self.pos.y + 1) * unsafe {SOLVER.size},
    //             Movement::Right => (self.pos.x - 1) + self.pos.y * unsafe {SOLVER.size},
    //             Movement::Left => (self.pos.x + 1) + self.pos.y * unsafe {SOLVER.size},
    //             Movement::No => self.pos.x + self.pos.y * unsafe {SOLVER.size}
    //         }
    //     });

    //     self.pos = match *movement {
    //         Movement::Right => Point {x: self.pos.x - 1, y: self.pos.y},
    //         Movement::Left => Point {x: self.pos.x + 1, y: self.pos.y},
    //         Movement::Down => Point {x: self.pos.x, y: self.pos.y - 1},
    //         Movement::Up => Point {x: self.pos.x, y: self.pos.y + 1},
    //         Movement::No => Point {x: self.pos.x, y: self.pos.y},
    //     };
    // }

    pub fn display(&self) {
        for y in 0..self.solver.size {
            let mut to_display = String::from("");
            for x in 0..self.solver.size {
                to_display.push_str(format!("{:4}", self.content[(x + y * self.solver.size) as usize]).as_str());
            }
            println!("{}\n", to_display);
        }
    }
}
