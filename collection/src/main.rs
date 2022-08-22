#![deny(clippy::all)]
fn main() {
    let values = ("Hello", "World", 30);

    let (_, _, age) = values;
    let value: [&str; 2] = ["foo", "Bar"];
    println!("{}, {}", value[0], value[1]);
}

fn get_values() -> (String, String, i32) {
    return ("Hello".to_string(), "world".to_string(), 32);
}
