#[derive(Clone, Copy, Debug, PartialEq)]
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
        if node == source {
            CellType::Source
        } else if node == target {
            CellType::Target
        } else {
            CellType::Clear
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

    pub fn set_parent(&mut self, node: Cell) {
        if (self.pos.x == node.x && (node.y == self.pos.y - 1 || node.y == self.pos.y + 1)) ||
           (self.pos.y == node.y && (node.x == self.pos.y - 1 || node.x == self.pos.y + 1)) {
            self.parent = Some(node);
        } else {
            panic!("Assigning not valid parent");
        }
    }

    pub fn set_child(&mut self, node: Cell) {
        if self.nchilds >= 4 {
            panic!("Trying to assign to many childs to a node");
        }
        for child in &self.childs {
            if child.is_some() && node == child.unwrap() {
                panic!("Trying to assign an already assigned child");
            }
        }
        if (self.pos.x == node.x && (node.y == self.pos.y - 1 || node.y == self.pos.y + 1)) ||
           (self.pos.y == node.y && (node.x == self.pos.y - 1 || node.x == self.pos.y + 1)) {
            self.childs[self.nchilds] = Some(node);
            self.nchilds += 1;
        } else {
            panic!("Assigning not valid child");
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

    pub fn get_node_mut(&mut self, node: Cell) -> Option<&mut Node> {
        let size = self.size;

        if node.x >= size || node.y >= size {
            None
        } else {
            Some(&mut self.content[node.x + node.y * size])
        }
    }

    pub fn get_node(&self, node: Cell) -> Option<&Node> {
        let size = self.size;

        if node.x >= size || node.y >= size {
            None
        } else {
            Some(&self.content[node.x + node.y * size])
        }
    }
}
