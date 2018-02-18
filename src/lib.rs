#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
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
    content: Vec<Node>,
    pub size: usize,
    pub source: Cell,
    pub target: Cell,
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

    pub fn check_child(&self, cell: Cell, map: &Map) -> bool {
        let node = map.get_node(cell);
        let mut res = false;

        if node.is_some() &&
           node.unwrap().ctype != CellType::Wall &&
           node.unwrap().ctype != CellType::Target &&
           (self.parent.is_none() || node.unwrap().pos != self.parent.unwrap()) {
            res = true
        }
        res
    }

    pub fn search_childs(&self, map: &Map) -> Vec<Cell> {
        let mut res = Vec::<Cell>::new();

        if self.pos.x > 0 {
            let left = Cell {x: self.pos.x - 1, y: self.pos.y};
            if self.check_child(left, map) {
                res.push(left);
            }
        }
        let right = Cell {x: self.pos.x + 1, y: self.pos.y};
        if self.check_child(right, map) {
            res.push(right);
        }
        if self.pos.y > 0 {
            let top = Cell {x: self.pos.x, y: self.pos.y - 1};
            if self.check_child(top, map) {
                res.push(top);
            }
        }
        let bottom = Cell {x: self.pos.x, y: self.pos.y + 1};
        if self.check_child(bottom, map) {
            res.push(bottom);
        }
        res
    }

    pub fn set_childs(&mut self, childs: Vec<Cell>) {
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
