use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Serialize, Debug)]
struct Attribute {
    hobby: String,
    game: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Human {
    name: String,
    age: i32,
    #[serde(flatten)]
    attr: Attribute,
    #[serde(flatten)]
    other: serde_json::Value,
}

#[derive(Deserialize, Serialize, Debug)]
struct HumanUnflattened {
    name: String,
    age: i32,
    attr: Attribute,
    other: serde_json::Value,
}

fn main() {
    let human = Human {
        name: "Bob".to_string(),
        age: 12,
        attr: Attribute {
            hobby: "Gaming".to_string(),
            game: "Dota".to_string(),
        },
        other: json!({
            "weight": 99,
            "age": 180,
        }),
    };
    let human_unflattened = HumanUnflattened {
        name: "Bob".to_string(),
        age: 12,
        attr: Attribute {
            hobby: "Gaming".to_string(),
            game: "Dota".to_string(),
        },
        other: json!({
            "weight": 99,
            "age": 180,
        }),
    };
    // serialize
    let human_ser = serde_json::to_string_pretty(&human).unwrap();
    let human_unflattened_ser = serde_json::to_string_pretty(&human_unflattened).unwrap();
    println!("human {}", human_ser);
    println!("human_unflattened {}", human_unflattened_ser);
}
