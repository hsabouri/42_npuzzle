#[derive(Debug, Clone, Copy)]
pub enum CellType {
    Source,
    Target,
    Wall,
    Clear,
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Node {
    pub content: CellType,
    pub parent: Option<Cell>,
    pub position: Cell,
    pub g: f32,
    pub h: f32,
    pub f: f32,
    pub childs: [Option<Cell>;4],
}

impl Node {
    pub fn new(content: CellType, parent: Option<Cell>, position: Cell, g: f32, h: f32, f: f32, childs: [Option<Cell>;4]) -> Node {
        Node {
            content: content,
            parent: parent,
            position: position,
            g: g,
            h: h,
            f: f,
            childs: childs,
        }
    }
}

pub mod heuristics {
    use tools::Cell;

    pub fn manhattan(pos1: Cell, pos2: Cell) -> f32 {
        (pos1.x as f32 - pos2.x as f32).abs() + (pos1.y as f32 - pos2.y as f32).abs()
    }

    pub fn euclidian(pos1: Cell, pos2: Cell) -> f32 {
        ((pos2.x as f32 - pos1.x as f32).powf(2.0) +
         (pos2.y as f32 - pos1.y as f32).powf(2.0)).sqrt()
    }
}
