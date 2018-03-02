extern crate rand;
mod lib;

fn get_childs(node: &mut lib::Node, solved: &lib::Map, index: usize) -> Vec<lib::Node> {
    let map = node.map.take().unwrap(); 
    let mut to_explore = Vec::<lib::Movement>::new();

    if map.pos.x < map.size - 1 && node.movement != lib::Movement::Right {
        to_explore.push(lib::Movement::Left);
    }
    if map.pos.x > 0 && node.movement != lib::Movement::Left {
        to_explore.push(lib::Movement::Right); 
    }
    if map.pos.y < map.size - 1 && node.movement != lib::Movement::Down {
        to_explore.push(lib::Movement::Up);
    }
    if map.pos.y > 0 && node.movement != lib::Movement::Up {
        to_explore.push(lib::Movement::Down); 
    }

    node.map = Some(map);
    to_explore.iter().map(|dir| node.child(*dir, index, solved)).collect()
}

fn push_sorted(mut list: Vec<lib::Node>, mut to_push: Vec<lib::Node>) -> Vec<lib::Node> {
    if list.len() == 0 {
        list.push(to_push.pop().unwrap());
    }

    let len = to_push.len();

    for _ in 0..len {
        let node = to_push.pop().unwrap();
        let index = list.binary_search(&node).unwrap_or_else(|e| e);

        list.insert(index, node);
    }
    list
}

fn main() {
    let nodes = lib::Node::gen(3);
    let solved = nodes.1.map.unwrap();
    let mut node = nodes.0;
    let mut openl = Vec::<lib::Node>::new();
    let mut closel = Vec::<lib::Node>::new();

    let childs = get_childs(&mut node, &solved, 0);

    closel.push(node);
    openl = push_sorted(openl, childs);
    while closel.last().unwrap().h > 0 {
        node = openl.remove(0);
        let childs = get_childs(&mut node, &solved, closel.len());

        closel.push(node);
        openl = push_sorted(openl, childs);
        println!("{:4} - {:4} - {:4}", closel.last().unwrap().f, closel.last().unwrap().h, closel.last().unwrap().g);
    }
    let mut i = closel.len();
    let mut last = closel.pop().unwrap();
    while i > 0 {
        let last_map = last.map.take().unwrap();

        last_map.display();
        println!("\n");
        i = last.parent;
        last = closel.remove(i);
    }
}
