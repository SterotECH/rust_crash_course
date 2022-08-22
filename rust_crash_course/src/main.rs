#![deny(clippy::all)]

fn main() {
    let hello = say_hello_world("John".to_string());
    println!("{}", hello);
    let _say_hello_world_inline = |name: &str| format!("Hello, {}", name);
    println!("{}", _say_hello_world_inline("World"));
    let full_name = |first_name: &str, last_name: &str| format!("{} {}", first_name, last_name);
    println!("{}", full_name("Samuel", "Agyei"));
    let _multiply_by_2 = |x: i32| x * 2;
    let _ask_for_age = || {
        // ask the user fir age
        // calculate how old in 10 years
    };

    let ptr = _multiply_by_2;
    let _result = ptr(90);
    println!("{}", _result);
}
fn say_hello_world(to_person: String) -> String {
    format!("Hello {}", to_person)
}

fn _process_name(name: &str, callback: fn(&str) -> ()) {
    callback(name);
}
