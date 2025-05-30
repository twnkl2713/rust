#![allow(unused)]
#[derive(Debug)]

struct Point {
    x: f32,
    y: f32,
}

struct Point3d(f32, f32, f32);

struct Empty;

struct Person {
  name: String,
  age: u32,
  can_vote: bool,
}

fn main(){
    // Create
    let p = Point { x: 1.0, y: 2.0 };
    println!("point.x = {}, point.y = {}", p.x, p.y);

    let p = Point3d(1.0, 2.0, 3.0);
    println!("point 3d-> {}, {}, {}", p.0, p.1, p.2);

    let empty = Empty;

    // Create a Person object
    let user = Person {
        name: String::from("Shinchan"),
        age: 5,
        can_vote: false,
    };

    // Access and print the values
    println!("Name: {}", user.name);
    println!("Age: {}", user.age);
    println!("Can vote? {}", user.can_vote);

    // Shortcut
    let x = 1.0;
    let y = 1.0;
    let p = Point{x, y};

    // Copy fields
    let p0 = Point{x: 1.0, y: 1.0};
    let p1 = Point{x: 2.0, ..p0};
    println!("{:?}", p0);
    println!("{:?}", p0);

    // Update
    let mut p = Point{x: 10.0, y:10.0};
    p.x += 1.0;
    p.y += 1.0;
    println!("{:?}", p);
}