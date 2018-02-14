mod lib;

fn main() {
    let mut map = lib::Map::new(3, lib::Cell {x: 0, y: 0}, lib::Cell {x: 2, y: 2});
    {
        let mut node_mut = map.get_node_mut(lib::Cell {x: 1, y: 1}).unwrap();
        
        node_mut.ctype = lib::CellType::Target;
        println!("{:#?}", node_mut)
    }
    let mut node = map.get_node(lib::Cell {x: 1, y: 1}).unwrap();
    println!("{:#?}", node)
}
