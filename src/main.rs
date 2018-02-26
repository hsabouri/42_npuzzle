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

fn main() {
    let mut node = lib::Node::gen(4);
    //let mut openl = Vec::<lib::Node>::new();
    let mut closel = Vec::<lib::Node>::new();
    let solved = node.1.map.unwrap();

    
    solved.display();
    println!("\n");
    let mut childs = get_childs(&mut node.0, &solved, 0);
    childs.iter().for_each(|x| {
        let map = x.map.as_ref().unwrap();
        map.display();
        println!("");
    });
    println!("\n");
    let mut childs2 = get_childs(&mut childs[1], &solved, 1);
    childs2.iter().for_each(|x| {
        let map = x.map.as_ref().unwrap();
        map.display();
        println!("");
    });
    closel.push(node.0);
    /*
    while closel.last().unwrap().f > 0 {
        
    }
    */
}
