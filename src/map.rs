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
    pub costs: Option<Vec<u16>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Heuristic {
    Manhattan,
    Naive,
    Linear,
}

impl Map {
    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        for _ in 0..100000 {
            let random_move: Movement = rng.gen();
            if random_move != Movement::No {
                if self.can_move(&random_move) {
                    self.do_move(&random_move);
                }
            }
        }
    }

    /// Check the validity of a map, will return an error if puzzle cannot be resolved.
    /// For mor information :
    /// http://www.cs.bham.ac.uk/~mdr/teaching/modules04/java2/TilesSolvability.html
    pub fn check_validity(&self) -> Result<(), &'static str> {
        let mut inversion: u32 = 0;
        for i in 0..self.solver.sq_size {
            if self.content[i] == 0 {
                continue;
            }
            for j in (i + 1)..self.solver.sq_size {
                if self.content[j] == 0 {
                    continue
                } else if self.content[j] > self.content[i] {
                    inversion += 1;
                }
            }
        }
        match self.solver.size % 2 {
            1 => match inversion % 2 {
                1 => Err("This puzzle cannot be solved"),
                _ => Ok(())
            },
            _ => match self.solver.size % 4 {
                0   => match inversion % 2 == (self.pos.y % 2) as u32 {
                    false  => Ok(()),
                    true => Err("This puzzle cannot be solved"),
                },
                _ => match inversion % 2 == (self.pos.y % 2) as u32 {
                    true  => Ok(()),
                    false => Err("This puzzle cannot be solved"),
                },
            },
        }
    }

    pub fn translate_in(&mut self) {
        self.content = self.solver.translate_in(&self.content);
    }
    pub fn translate_out(&mut self) {
        self.content = self.solver.translate_out(&self.content);
    }

    pub fn new(content: Vec<u16>, ref solver: &'static Solver, pos: Point, costs: Option<Vec<u16>>) -> Map {
        Map {content: content, solver: solver, pos: pos, costs: costs}
    }

    fn can_move(&self, direction: &Movement) -> bool {
        match direction {
            &Movement::Up => self.pos.y > 0,
            &Movement::Down => self.pos.y < (self.solver.size - 1),
            &Movement::Left => self.pos.x > 0,
            &Movement::Right => self.pos.x < (self.solver.size - 1),
            &Movement::No => true,
        }
    }

    fn do_move(&mut self, direction: &Movement) {
        self.content.swap((self.pos.x + self.pos.y * self.solver.size) as usize,
        (match direction {
            &Movement::Up => self.pos.x + (self.pos.y - 1) * self.solver.size,
            &Movement::Down => self.pos.x + (self.pos.y + 1) * self.solver.size,
            &Movement::Left => (self.pos.x - 1) + self.pos.y * self.solver.size,
            &Movement::Right => (self.pos.x + 1) + self.pos.y * self.solver.size,
            &Movement::No => self.pos.x + self.pos.y * self.solver.size
        }) as usize
        );

        self.pos = match direction {
            &Movement::Up => Point {x: self.pos.x, y: self.pos.y - 1},
            &Movement::Down => Point {x: self.pos.x, y: self.pos.y + 1},
            &Movement::Left => Point {x: self.pos.x - 1, y: self.pos.y},
            &Movement::Right => Point {x: self.pos.x + 1, y: self.pos.y},
            &Movement::No => Point {x: self.pos.x, y: self.pos.y},
        };
    }

    pub fn child(&self, movement: &Movement) -> Option<Map> {
        match self.can_move(movement) {
            true => {
                let mut res = self.clone();

                res.do_move(movement);
                res.set_costs(movement);
                Some(res)
            },
            false => None,
        }
    }

    fn conflict(&self, costs: &Vec<u16>, start: &Point, end: &Point) -> u16 {
        let mut res: u16 = 0;

        if start.x == end.x {
            for i in start.y..end.y {
                let index = self.solver.point_to_index(&Point {x: start.x, y: i});

                if costs[index as usize] == 0 {
                    res += 1;
                }
            }
        } else if start.y == end.y {
            for i in start.x..end.x {
                let index = self.solver.point_to_index(&Point {x: i, y: start.y});

                if costs[index as usize] == 0 {
                    res += 1;
                }
            }
        }
        res
    }

    fn first_linear(&self) -> Vec<u16> {
        let mut res: Vec<u16> = self.first_naive();

        for (index, value) in self.content.iter().enumerate() {
            let solved_index = self.solver.from_value_to_index(*value as u16);
            let value_pos = self.solver.index_to_point(index as u16);
            let solved_pos = self.solver.index_to_point(solved_index as u16);
            let cost = ((value_pos.x as i16 - solved_pos.x as i16).abs() + (value_pos.y as i16 - solved_pos.y as i16).abs()) as u16 * 10;
            let occurences = self.conflict(&res, &value_pos, &solved_pos);

            res[*value as usize] = cost + occurences * 2;
        }
        res
    }

    fn linear(&mut self, mov: &Movement) -> Vec<u16> {
        let size = self.solver.size;
        let zero_pos = &self.solver.zero_pos;
        let to_look_at = match *mov {
            Movement::Up => self.pos.x + (self.pos.y + 1) * size,
            Movement::Down => self.pos.x + (self.pos.y - 1) * size,
            Movement::Left => self.pos.x + 1 + self.pos.y * size,
            Movement::Right => self.pos.x - 1 + self.pos.y * size,
            Movement::No => self.pos.x + self.pos.y * size,
        };
        let value = self.content[to_look_at as usize];
        let solved_pos = self.solver.index_to_point(self.solver.from_value_to_index(value));
        let value_pos = self.solver.index_to_point(to_look_at);
        let mut costs = self.costs.take().unwrap();

        let cost = ((value_pos.x as i16 - solved_pos.x as i16).abs() + (value_pos.y as i16 - solved_pos.y as i16).abs()) as u16 * 10;
        let zero_cost = ((self.pos.x as i16 - zero_pos.x as i16).abs() + (self.pos.y as i16 - zero_pos.y as i16).abs()) as u16 * 10;
        let occurences = self.conflict(&costs, &value_pos, &solved_pos);
        let zero_occurences = self.conflict(&costs, &zero_pos, &self.pos);

        costs[value as usize] = cost + occurences * 2;
        costs[0] = zero_cost + occurences * 2;
        costs
    }

    fn naive(&mut self, mov: &Movement) -> Vec<u16> {
        let size = self.solver.size;
        let zero_index = self.solver.zero_index;
        let to_look_at = match *mov {
            Movement::Up => self.pos.x + (self.pos.y + 1) * size,
            Movement::Down => self.pos.x + (self.pos.y - 1) * size,
            Movement::Left => self.pos.x + 1 + self.pos.y * size,
            Movement::Right => self.pos.x - 1 + self.pos.y * size,
            Movement::No => self.pos.x + self.pos.y * size,
        };
        let solved_value = self.solver.from_index_to_value(to_look_at);
        let value = self.content[to_look_at as usize];
        let mut costs = self.costs.take().unwrap();

        if value == solved_value {
            costs[value as usize] = 0;
        } else {
            costs[value as usize] = 1;
        }
        if zero_index == self.solver.point_to_index(&self.pos) {
            costs[0] = 0;
        } else {
            costs[0] = 1;
        }
        costs
    }

    fn first_naive(&self) -> Vec<u16> {
        let mut res: Vec<u16> = vec![0; self.solver.sq_size];

        for (index, value) in self.content.iter().enumerate() {
            let solved_value = self.solver.from_index_to_value(index as u16);

            if solved_value == *value {
                res[(*value) as usize] = 0;
            } else {
                res[(*value) as usize] = 10;
            }
        }
        res
    }

    fn manhattan(&mut self, mov: &Movement) -> Vec<u16> {
        let size = self.solver.size;
        let zero_pos = &self.solver.zero_pos;
        let to_look_at = match *mov {
            Movement::Up => self.pos.x + (self.pos.y + 1) * size,
            Movement::Down => self.pos.x + (self.pos.y - 1) * size,
            Movement::Left => self.pos.x + 1 + self.pos.y * size,
            Movement::Right => self.pos.x - 1 + self.pos.y * size,
            Movement::No => self.pos.x + self.pos.y * size,
        };
        let value = self.content[to_look_at as usize];
        let solved_pos = self.solver.index_to_point(self.solver.from_value_to_index(value));
        let value_pos = self.solver.index_to_point(to_look_at);
        let mut costs = self.costs.take().unwrap();

        costs[value as usize] = ((value_pos.x as i16 - solved_pos.x as i16).abs() + (value_pos.y as i16 - solved_pos.y as i16).abs()) as u16 * 10;
        costs[0] = ((self.pos.x as i16 - zero_pos.x as i16).abs() + (self.pos.y as i16 - zero_pos.y as i16).abs()) as u16 * 10;
        costs
    }

    fn first_manhattan(&self) -> Vec<u16> {
        let mut res: Vec<u16> = vec![0; self.solver.sq_size];

        for (index, value) in self.content.iter().enumerate() {
            let solved_index = self.solver.from_value_to_index(*value as u16);
            let value_pos = self.solver.index_to_point(index as u16);
            let solved_pos = self.solver.index_to_point(solved_index as u16);
            let cost = ((value_pos.x as i16 - solved_pos.x as i16).abs() + (value_pos.y as i16 - solved_pos.y as i16).abs()) as u16 * 10;

            res[*value as usize] = cost;
        }
        res
    }

    pub fn set_first_costs(&mut self) {
        let func = self.solver.func;

        self.costs = Some(match func {
            Heuristic::Naive => self.first_naive(),
            Heuristic::Manhattan => self.first_manhattan(),
            _ => self.first_linear(),
        });
    }

    pub fn set_costs(&mut self, mov: &Movement) {
        let func = self.solver.func;

        self.costs = Some(match func {
            Heuristic::Naive => self.naive(mov),
            Heuristic::Manhattan => self.manhattan(mov),
            _ => self.linear(mov),
        });
    }

    pub fn get_cost(&mut self) -> u16 {
        let costs = self.costs.take().unwrap();
        let res = costs.iter().fold(0, |mut sum, &val| {sum += val; sum});
        self.costs = Some(costs);
        res
    }

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
