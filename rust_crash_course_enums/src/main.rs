#![deny(clippy::all)]
#[derive(PartialEq)]

struct Size {
    width: f32,
    height: f32,
}
enum Shapes {
    Rectangle(f32, f32, Size),
    Circle(f32, f32, f32),
}

enum Pet {
    Cat { name: String },
    Dog { name: String },
}
impl Shapes {
    fn area(&self) -> f32 {
        match self {
            Shapes::Rectangle(_x, _y, size) => size.width * size.height,
            Shapes::Circle(_x, _y, radius) => std::f32::consts::PI * radius * radius,
        }
    }
}

fn main() {
    let rectangle = Shapes::Rectangle(
        10.0,
        10.0,
        Size {
            width: 3.0,
            height: 4.0,
        },
    );
    if let Shapes::Rectangle(x, y, Size { width, height }) = rectangle {
        println!(
            "x = {} y = {} size (height = {} width = {} )",
            x, y, width, height
        )
    }

    let area = rectangle.area();
    println!("Area is {}", area);

    let circle = Shapes::Circle(10.0, 20.0, 7.0);
    let area = circle.area();
    println!("Area is {}", area);

    let fluffy = Pet::Cat {
        name: "Fluffy".to_string(),
    };

    let doggy = Pet::Dog {
        name: "Black".to_string(),
    };
    let name = match fluffy {
        Pet::Cat { name } => name,
        Pet::Dog { name } => name,
    };

    let name = match doggy {
        Pet::Cat { name } => name,
        Pet::Dog { name } => name,
    };
    println!("Hello {} ", name);
}
