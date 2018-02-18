mod lib;

fn solve(map: &mut lib::Map) {
    let source_pos = map.source;
    let childs = map.get_node(source_pos).unwrap().search_childs(map);
    let mut source = map.get_node_mut(source_pos).unwrap();

    source.set_childs(childs);
}

fn main() {
    let mut map = lib::Map::new(4, lib::Cell {x: 0, y: 0}, lib::Cell {x: 2, y: 2});

    solve(&mut map);

    println!("{:#?}", map);
}
