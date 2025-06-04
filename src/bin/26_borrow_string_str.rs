#![allow(unused)]

// String and str
fn take_string(s: String) {}
fn borrow_string(s: &String) {} // Immutable borrow of String
fn make_string() -> String {
    "".to_string()
}

// str - size not known at compile time - so can't be used for func i/p & o/p
fn borrow_str(s: &str) {} // Immutable borrow of string slice (&str)

fn mut_string(s: &mut String) {  // Mutable borrow of String
    s.push_str("?");
}
fn main() {
    // String
    let s = String::from("rust");
    take_string(s);
    // println!("{s}"); - this won't compile

    // mut string
    let mut s = String::from("rust");
    s += "!";
    println!("mut String: {s}");

    // &String
    let s = String::from("rust");
    let s1: &String = &s;
    borrow_string(s1); // borrow as &String
    borrow_str(&s); // can be coerced into &str
    println!("&String: {s}");

    // &mut String
    let mut s = String::from("rust");
    let s1: &mut String = &mut s;
    s1.push_str("!");
    println!("&mut String: {s}");

    // str - string slice
    // can't create a variable of type str or use for func i/p
    // let s: str = "yo";

    // &str - size known at compile time(pointer) and immutable borrow
    let s: &str = "hello";
    borrow_str(s);
    println!("&str: {s}");

    // &mut str - possible to create &mut str but uncommon. (use mut str)
    let mut s: String = "hello".to_string();
    let s1: &mut str = &mut s; 
}