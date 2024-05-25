#[derive(Default, Debug)]
struct Data {
    a: i32,
    b: Option<i32>,
    c: Option<i32>,
    d: Option<i32>,
    e: Option<i32>,
}

fn main() {
    let data = Data {
        a: 3,
        ..Default::default()
    };
    println!("{:?}", data);
}
