#![allow(unused)]

// Every reference has a lifetime

// Both x and y live at least 'a
fn longest_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/*
// Multiple lifetime
fn print_refs<'a, 'b>(x: &'a str, y: &'b str) {
    println!("{} {}", x, y);
}
*/

fn main() {
    let x = "hello".to_string();
    let y = "rust".to_string();
    let z = longest_str(x.as_str(), y.as_str());
    println!("longest {:?}", z);
}