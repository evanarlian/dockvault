use serde_json::json;

fn main() {
    let mut value = json!({
        "credHelper": ["a"],
    });
    match value.get("auths") {
        Some(v) => println!("{:?}", v),
        None => {
            let vv = value.as_object_mut().unwrap();
            vv.insert("auths".to_string(), serde_json::Value::from(3));
        }
    }
    dbg!(value);
}
