#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Copy, Debug)]
pub enum CellType {
    Clear,
    Wall,
    Source,
    Target,
}

impl CellType {
    pub fn figure_out(node: Cell, source: Cell, target: Cell) -> CellType {
        match node {
            target if target.x == node.x && target.y == node.y => CellType::Target,
            source if source.x == node.x && source.y == node.y => CellType::Source,
            _ => CellType::Clear,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Node {
    pub pos: Cell,
    pub ctype: CellType,
    pub parent: Option<Cell>,
    pub nchilds: usize,
    pub childs: [Option<Cell>; 4],
    g: Option<f32>,
    h: Option<f32>,
    f: Option<f32>,
}

#[derive(Debug)]
pub struct Map {
    content:    Vec<Node>,
    pub size:   usize,
    source:     Cell,
    target:     Cell,
}

impl Node {
    pub fn new(pos: Cell, ctype: CellType) -> Node {
        Node {
            pos: pos,
            ctype: ctype,
            parent: None,
            nchilds: 0,
            childs: [None, None, None, None],
            g: None,
            h: None,
            f: None,
        }
    }
}

impl Map {
    pub fn new(size: usize, source: Cell, target: Cell) -> Map {
        if source.x >= size || source.y >= size {
            panic!("Source is out of the map");
        }
        if target.x >= size || target.y >= size {
            panic!("Target is out of the map");
        }
        Map {
            content: {(0..size * size).map(|i|{
                let pos = Cell {x: i % size , y: i / size};

                Node::new(pos, CellType::figure_out(pos, source, target))
            }).collect()},
            size: size,
            source: source,
            target: target,
        }
    }
}
