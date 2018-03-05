use super::Map;

pub fn parse(filename: &str) -> Result<(Map, u16), &'static str> {
    // Ok(Node::gen(3))
    Ok((Map::new_random(3), 3))
}
