use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    age: Option<i32>,
}

fn main() {
    let data = Data {
        username: Some(String::from("asd")),
        age: Some(4),
    };
    let empty = Data {
        username: None,
        age: None,
    };
    let data_ser = serde_json::to_string_pretty(&data).unwrap();
    println!("{}", data_ser);
    let empty_ser = serde_json::to_string_pretty(&empty).unwrap();
    println!("{}", empty_ser);
}
