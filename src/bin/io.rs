#![allow(unused)]

use std::io;

fn main() {
    let mut input = String::new();
    println!("Please input your name: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    println!("User input: {}", input);
}