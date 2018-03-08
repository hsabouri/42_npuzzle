
use generator;

#[derive(Debug, Clone, Eq, PartialEq)]// TODO why all this stuff on eq etc ... ?
pub struct Solver {
    pub size: u16,
    pub sq_size: usize,
    pub zero_pos: u16,
    pub solved: Option<Vec<u16>>
}

static mut SOLVER: Solver = Solver {
    size: 0,
    sq_size: 0,
    zero_pos:0,
    solved: None
};

impl Solver {
    pub fn new(size: u16) -> &'static Solver {
        unsafe {
            SOLVER.size = size;
            SOLVER.sq_size = (size * size) as usize;
            SOLVER.zero_pos = (size / 2) * (size + 1) + size % 2 - 1;
            let mut vec = generator::create_solved_spiral(size as i16);//TODO import generator here
            vec[SOLVER.zero_pos as usize] = 0;
            SOLVER.solved = Some(vec);
            &SOLVER
        }
    }
    fn vec_index_to_array_index(&self, index: u16) -> u16 {
        if index < self.zero_pos {
            index + 1
        } else if index > self.zero_pos {
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
}
