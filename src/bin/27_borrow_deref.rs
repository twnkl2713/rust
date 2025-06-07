#![allow(unused)]

fn modify(s: &mut String) {
    // this doesn't take ownership
    *s += "?";
}

fn main() {
    // Deref
    let mut s = String::from("dundun");
    let s1 = &mut s;
    *s1 += "?";
    println!("{s}");

    let mut s = String::from("dunny");
    modify(&mut s);
    println!("{s}");

    // deref coercion
    // automatically dereferenced in some situations
    let x = 1;
    let y = &x;
    let z = &y;
    let w = *y + **z;
    println!("w = {w}");
}