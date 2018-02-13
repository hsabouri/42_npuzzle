mod tools;

static MAP_SIZE: usize = 10;
static TARGET: tools::Cell = tools::Cell {x: 8, y: 1};
static SOURCE: tools::Cell = tools::Cell {x: 1, y: 8};

fn get_node(node: tools::Cell) -> usize {
    node.x + node.y * MAP_SIZE
}

fn set_child(node: tools::Cell, child: tools::Cell, memory: &mut Vec<tools::Node>) {
    let candidate = memory[get_node(child)];

    if candidate.parent.is_none() || candidate.childs[0].is_none() {
        let id: usize = {
            let mut i: usize = 0;
            while memory[get_node(node)].childs[i].is_some() {i += 1;}
            i
        };
        memory[get_node(node)].childs[id] = Some(child);
    }
}

fn set_childs(node: tools::Cell, memory: &mut Vec<tools::Node>) {
    if node.x as i32 - 1 >= 0 {
        set_child(node, tools::Cell {x: node.x - 1, y: node.y}, memory);
    }
    if node.x as i32 + 1 < MAP_SIZE as i32 {
        set_child(node, tools::Cell {x: node.x + 1, y: node.y}, memory);
    }
    if node.y as i32 - 1 >= 0 {
        set_child(node, tools::Cell {x: node.x, y: node.y - 1}, memory);
    }
    if node.y as i32 + 1 < MAP_SIZE as i32 {
        set_child(node, tools::Cell {x: node.x, y: node.y + 1}, memory);
    }
}

fn explore(target: tools::Cell, start: tools::Cell, memory: &mut Vec<tools::Node>) {
    if (memory[get_node(node)].childs[0].is_none()) {
        set_childs(start, memory);
    }
    
}

fn main() {
    let mut memory: Vec<tools::Node> = (0..MAP_SIZE * MAP_SIZE).map(|i| {
        let (x, y) = (i % MAP_SIZE, i / MAP_SIZE);

        tools::Node::new(
            {match (x, y) {
                (x, y) if x == SOURCE.x && y == SOURCE.y => tools::CellType::Source,
                (x, y) if x == TARGET.x && y == TARGET.y => tools::CellType::Target,
                _ => tools::CellType::Clear,
            }},
            None,
            tools::Cell {x: x, y: y},
            tools::heuristics::manhattan(TARGET, tools::Cell {x: x, y: y}),
            0.0,
            0.0,
            [None;4],
        )
    }).collect();

    //println!("Memory : {:#?}", memory);
    //println!("Close List : {:#?}", close_list);
}
