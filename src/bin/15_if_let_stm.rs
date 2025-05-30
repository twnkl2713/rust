#![allow(unused)]

fn main() {
    let x: Option<u32> = Some(314);
    match x {
        Some(v) => println!("Some {v}"),
        _ => {}
    }
    // if let
    if let Some(v) = x {
        println!("if let {v}");
    }
    // let else 
    let Some(v) = x else {
        // diverge - panic pr return
        panic!("x is none");
    };
    println!("if let {v}");
}