#![allow(unused)]

// closure as function output

// move keyword must be used which signals that all captures occur by value

fn fn_out() -> impl Fn(u32) -> u32 {
    |x| x + 1
}

fn fn_out_move() -> impl Fn() {
    let s = "hello".to_string();
    // must move ownership into closure
    move || {
        println!("fn_out_move: {s}");
    }
}

fn fn_mut_out() -> impl FnMut() {
    let mut s = "hello".to_string();
    move || {
        s += " world!";
        println!("fn_mut_out: {}", s);
    }
}

fn fn_mut_return_copy() -> impl FnMut() -> u32 {
    let mut x = 0;
    move || {
        x += 1;
        x // x is copied so that we can return it
    }
}

fn fn_once_out() -> impl FnOnce() -> String {
    let s = "hello".to_string();
    move || {
        println!("fn_once_out: {}", s);
        s // can return s because this closure can only be called once
    }
}

fn main() {
    // Fn
    let f = fn_out();
    // call more than once
    let z = f(1);
    println!("main: z = {z}");
    let z = f(2);
    println!("main: z = {z}");

    // Fn move
    let f = fn_out_move();
    f();
    f();

    // FnMut
    let mut f = fn_mut_out();
    f();
    f();

    // FnMut return value
    let mut f = fn_mut_return_copy();
    let z = f();
    println!("z: {z}");
    let z = f();
    println!("z: {z}");
    let z = f();
    println!("z: {z}");

    let f = fn_once_out();
    let s = f();
    println!("main: {s}");
    // f(); -> can't call twice
}