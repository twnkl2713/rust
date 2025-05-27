#![allow(unused)]
#[derive(Debug)]
#[derive(PartialEq)]

// Enum is a way to define a type that can be one of a few different values
enum Color{ // each value in the enum is called a variant
    Red,
    Green,
    Blue,
    Rgba(u8, u8, u8, f32),
    Hex(String),
    Hsl { h: u8, s: u8, l: u8 },
}

enum Direction {
  Up,
  Down,
  Left,
  Right,
}
fn main(){
    // Enum
    let color = Color::Red;
    let color = Color::Green;
    let color = Color::Rgba(0, 0, 255, 0.1);
    let color = Color::Hex("#ffffff".to_string());
    let color = Color::Hsl { h: 0, s: 1, l: 200 };

    // Attributes - Debug and PartialEq
    // Debug
    println!("{:?}", color);
    
    let my_direction = Direction::Up;
    println!("We are going up!");

    // PartialEq
    println!("{}", Color::Red == Color::Green);
    println!("{}", Color::Red == Color::Red);

    // Option = Some(11) | Node;
    let x :Option<i32> = None;
    let x :Option<i32> = Some(-11);
    println!("option: {:?}", x);

    // Result = Ok(5) | Err("div by 0")
    let res: Result<u32, String> = Ok(5);
    let res: Result<u32, String> = Err("div by 0".to_string());
    println!("result: {:?}", res);
}