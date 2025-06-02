#![allow(unused)]

// Function to take ownership of a String
fn take(s: String) {
    println!("take: {s}");
    // s is dropped here when the function ends
}

// Function to copy an integer (which implements Copy trait)
fn copy(i: i32) {
    println!("copy: {i}");
    // i is copied and dropped here
}

fn main(){
    // ownership is needed because -
    // memory - stack and heap
    // Stack 
    // - stores data of fixed size at compile time
    // - fast
    // - LIFO 
    // Heap
    // - Stores data of unknown size at compile time
    // - Slower than stack
    // - Data managed by ownership and borrowing rules

    // ownership rules -
    // 1. each value has an owner
    // 2. there can only be one owner at a time
    // 3. when the owner goes out of scope, the value will be dropped

    // 1. each value has an owner
    // owner of "dundun chicken" is s
    let s = String::from("dundun chicken");
    // owner of -1 is i
    let i: i32 = -1;

    // 2. there can only be one owner at a time
    let s = String::from("dundun chicken");
    // Owner of "dundun chicken" is s1
    let s1 = s;
    // Owner of "dundun chicken" is s2
    let s2 = s1;
    // This will not compile
    // println!("{s}");
    println!("{s2}");

    // ownership doesn't move for types that implement the Copy trait.
    // values are copied, separate owners for i, i1 an i2
    // owner of i is i
    let i = 1;
    // owner of i1 is i1
    let i1 = i;
    // owner of i2 is i2
    let i2 = i1;
    println!("i = {i}, i1 = {i1}, i2 = {i2}");

    // 3. when the owner goes out of scope, the value will be dropped
    let s = String::from("dunny fox");
    {
        s;
    }
    // This will not compile
    // println!("{s}");

    let s = String::from("dunny fox");
    {
        // Owner of s is s1
        let s1 = s;
        println!("{s1}");
        // s1 is dropped
    }
    // This will not compile
    // println!("{s}");

    let s = String::from("dunny fox");
    take(s);
    // This will not compile
    // println!("{s}");

    let i = 1;
    // i is copied as function input
    copy(i);
    // i is not dropped so this compiles
    println!("i = {i}");
}