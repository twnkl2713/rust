#![allow(unused)]

// CALLING A FUNCTION
fn say_hello() {
  println!("Hello Twinkle!");
}

// FUNCTIONS WITH PARAMETERS
fn greet(name: &str) {
    println!("Yo {}!", name);
}

// FUNCTION WITH RETURN VALUES
// -> is used to show what type of value will be returned
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
fn mult(x: i32, y: i32) -> i32 {
    return x * y // Implicit return, no semicolon
}

// NO OUTPUT
fn print_name(name: String) {
    println!("{} Rust", name);
}

// Diverge - never return
fn forever() -> ! {
    loop {}
}

fn crash() -> ! {
    panic!("error");
}

fn main() {
    say_hello();
    greet("Mango pudding");
    let sum = add(3, 4);
    let x = 3;
    let y = 2;
    let multi = mult(x, y);
    println!("Sum is: {}", sum);
    println!("Multiplication is: {x} * {y} = {}", multi);
    print_name("Kon'nichiwa".to_string());
}
