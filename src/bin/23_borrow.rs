#![allow(unused)]

fn main(){
    // Immutable borrow
    let s = String::from("Dunny Fox");
    // s1, s2, and s3 have read-only access to s
    let s1 = &s;
    let s2 = &s;
    let s3 = &s;

    // Borrow - temporarily use a value without taking ownership
    // - Creates a reference (either mutable or immutable)
    // - Doesn't move ownership
    // - Immutable reference - any number of read-only access to a value
    // - Mutable reference - only one read and write access to a value at a time
    // - Either immutable or mutable borrow, but not both simultaneously.
    // - Reference must not outlive the value

    // Mutable borrow - eg 1
    let mut s = String::from("DUNNY ");
    // s1 has read and write access to s
    let s1 = &mut s;
    // This will not compile - multiple mutable reference to s at the same time
    // let s2 = &mut s;
    s1.push_str("FOX");
    println!("{s}");

    // Mutable borrow - example 2
    // This will compile - multiple mutable reference to s but not simultaneously
    let mut s = String::from("Dun ");
    let s1 = &mut s;
    s1.push_str("Dun ");
    // s1 is no longer used below
    // so there is only one mutable reference (s2)
    let s2 = &mut s;
    s2.push_str("Chicken");
    println!("{s}");

    // Cant borrow immutable and mutable simultaneously
    let mut s = String::from("rust");
    let s1 = &s;
    let s2 = &s;
    // let s3 = &mut s;
    println!("s1 {s1}");

    // Reference must not outlive the value
    let s = String::from("shinchan");
    let s1 = &s;
    // Examples below will not compile
    // Example 1 - Transfer of ownership to s2 and then drop s2
    // {
    //     let s2 = s;
    // }
    // Example 2 - force s to drop
    // std::mem::drop(s);
    // Example 3 - see the function dangle
    println!("s1 {s1}");
}