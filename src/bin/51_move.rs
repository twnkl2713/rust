#![allow(unused)]

fn f(s: &mut String) {}

fn main() {
    let mut s = "rust".to_string();
    f(&mut s);
    println!("{s}");

    // closures can capture variables by 
    // -borrow immutable reference &T
    // -borrow mutable reference &mut T
    // -take ownership of value T

    // Borrow immutable reference &T
    let s = "rust".to_string();
    let f = || println!("borrow: {s}");
    f();
    println!("main: {s}");

    // Borrow mutable reference &mut T
    let mut s = "rust".to_string();
    let mut f = || s += " world";
    f();
    println!("main: {s}");

    // Take ownership of value T
    let s = "rust".to_string();
    let f = move || {
        println!("move: {s}");
        s;
    };
    f();
    f();
}