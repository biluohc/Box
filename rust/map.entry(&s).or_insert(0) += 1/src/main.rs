use std::collections::BTreeMap as Map;

fn main() {
    let s = "0,1,7,3,5,6,7,8,9,8,10,5,3,2,6,9,,3";
    let mut map = Map::new();
    s.split(",")
        .filter(|ss| !ss.trim().is_empty())
        .map(|ss| *map.entry(ss).or_insert(0) += 1)
        .count();
    println!("{:?}", map);
}
