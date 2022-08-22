#![deny(clippy::all)]

// Regular Struct
struct Person {
    name: String,
    age: u8,
    mothers_name: String,
}

//tuple Struct
#[derive(Debug)]
struct Point(f64, f64, f64);

/// implementation of struct
impl Point {
    fn describe(&self) {
        println!(
            "Point is at (x = {}, y = {}, z = {})",
            self.0, self.1, self.2
        );
    }
    fn _twice(&self) -> Point {
        Point(self.0 * 2.0, self.1 * 2.0, self.2 * 2.0)
    }

    fn _make_twice(&mut self) {
        self.0 *= 2.0;
        self.1 *= 2.0;
        self.2 *= 2.0;
    }

    fn _zero() -> Point {
        Point(0.0, 0.0, 0.0)
    }
}
fn main() {
    let person = Person {
        name: "Agyei Samuel".to_string(),
        age: 31,
        mothers_name: "Joan".to_string(),
    };

    println!(
        "{} You are {} years old. You mother is {}",
        person.name, person.age, person.mothers_name
    );
    // Struct Update Syntax
    let _person2 = Person {
        name: "Coachella".to_string(),
        ..person
    };

    // instantiate tuple struct
    let point = Point(1.0, 2.0, 0.1);
    println!("x = {}, y = {}, z = {}", point.0, point.1, point.2);

    point.describe();
    println!("{:?}", point);

    let mut point = Point(1.0, 2.0, 3.0);
    let twice = point._twice();
    println!("{:?}", twice);
    point._make_twice();
    let point = Point::_zero();
    println!("{:#?}", point);
}

fn _create_person(name: String, age: u8, mothers_name: String) {
    let _person = Person {
        name,
        age,
        mothers_name,
    };
}
