use std::collections::BTreeMap;

fn main() {
    let mut bmap = BTreeMap::new();
    bmap.insert(String::from("zzz"), 3);
    bmap.insert(String::from("a"), 3);
    bmap.insert(String::from("g"), 3);
    dbg!(&bmap);
    for (i, (k, v)) in bmap.iter().enumerate() {
        println!("{} {} {}", i, k, v);
    }
}
