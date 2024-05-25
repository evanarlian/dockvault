use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
struct Entry {
    auth: String,
}

#[derive(Serialize, Deserialize)]
struct Big {
    auths: Vec<Entry>,
}

fn main() {
    // prep
    let e1 = Entry {
        auth: String::from("entry_value1"),
    };
    let e2 = Entry {
        auth: String::from("entry_value2"),
    };
    let e3 = Entry {
        auth: String::from("entry_value3"),
    };
    let big = Big {
        auths: vec![e1, e2, e3],
    };
    for entry in &big.auths {
        println!("{}", entry.auth);
    }

    // serde
    let result = serde_json::to_string_pretty(&big).unwrap();
    println!("{}", result);
}
