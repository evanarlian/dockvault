use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    balls: BTreeSet<String>
}

fn main() {
    let mut balls = BTreeSet::new();
    balls.insert(String::from("bbb"));
    balls.insert(String::from("aaa"));
    balls.insert(String::from("aaa"));
    balls.insert(String::from("aaa"));
    balls.insert(String::from("ccc"));
    let data = Data {balls};
    let data_ser = serde_json::to_string_pretty(&data).unwrap();
    println!("{}", data_ser);
}