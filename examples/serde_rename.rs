use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    #[serde(rename = "Name")]
    name: String,
    age: i32,
}

fn main() {
    let raw = json!({
        "Name": "alice",
        "age": 3,
    });
    let result: Data = serde_json::from_value(raw).unwrap();
    dbg!(&result);
    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}
