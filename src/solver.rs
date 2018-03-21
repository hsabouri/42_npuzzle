use generator;
use map::{Heuristic, Point};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]// TODO why all this stuff on eq etc ... ?
pub struct Solver {
    pub size: u16,
    pub sq_size: usize,
    pub zero_index: u16,
    pub zero_pos: Point,
    pub func: Heuristic,
    pub solved: Option<Vec<u16>>,
    pub boost: u16,
}

static mut SOLVER: Solver = Solver {
    size: 0,
    sq_size: 0,
    zero_index: 0,
    zero_pos: Point {x: 0, y: 0},
    func: Heuristic::Manhattan,
    solved: None,
    boost: 1,
};

impl Solver {
    pub fn new(size: u16, func: Heuristic, boost: u16) -> &'static Solver {
        unsafe {
            SOLVER.size = size;
            SOLVER.sq_size = (size * size) as usize;
            SOLVER.zero_index = (size / 2) * (size + 1) + size % 2 - 1;
            SOLVER.zero_pos = Point {x: SOLVER.zero_index % SOLVER.size, y: SOLVER.zero_index / SOLVER.size};
            SOLVER.func = func;
            let mut vec = generator::create_solved_spiral(size as i16);//TODO import generator here
            vec[SOLVER.zero_index as usize] = 0;
            SOLVER.solved = Some(vec);
            SOLVER.boost = boost;
            &SOLVER
        }
    }
    fn array_value_to_spiral_index(&self, value: u16) -> u16 {
        if value == 0 {
            self.zero_index
        } else if value <= self.zero_index {
            value - 1
        } else {
            value
        }
    }
    fn vec_index_to_array_index(&self, index: u16) -> u16 {
        if index < self.zero_index {
            index + 1
        } else if index > self.zero_index {
            index
        } else {
            0
        }
    }
    fn spiral_value_to_vec_index(&self, value: u16) -> u16 {
        if let Some(ref table) = self.solved {
            for i in 0..self.sq_size {
                if value == table[i] {
                    return i as u16;
                }
            }
        }
        panic!("There is no {} in solved", value);
    }

    pub fn translate_in(&self, spiral_vec: &Vec<u16>) -> Vec<u16> {
        let mut map: Vec<u16> = vec![0; self.sq_size];
        for i in 0..self.sq_size {
            map[i] = self.vec_index_to_array_index(self.spiral_value_to_vec_index(spiral_vec[i]));
        }
        map
    }

    pub fn translate_out(&self, array_vec: &Vec<u16>) -> Vec<u16> {
        let mut map: Vec<u16> = vec![0; self.sq_size];
        if let Some(ref table) = self.solved {
            for i in 0..self.sq_size {
                map[i] = table[self.array_value_to_spiral_index(array_vec[i]) as usize];
            }
        }
        map
    }

    pub fn from_index_to_value(&self, index: u16) -> u16 {
        let zero_index = self.zero_index;

        if index < zero_index {
            index + 1
        } else if index > zero_index {
            index
        } else {
            0
        }
    }

    pub fn from_value_to_index(&self, value: u16) -> u16 {
        let zero_index = self.zero_index;

        if value == 0 {
            zero_index
        } else if value <= zero_index {
            value - 1
        } else {
            value
        }
    }

    pub fn point_to_index(&self, point: &Point) -> u16 {
        let size = self.size;

        point.x + point.y * size
    }

    pub fn index_to_point(&self, index: u16) -> Point {
        Point {
            x: index % self.size,
            y: index / self.size
        }
    }
}
