#![allow(unused)]

fn main(){
    // Tuple
    let person = ("Shinchan", 5, true); // this contains a &str, an i32 and a bool
    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    println!("Is active: {}", person.2);

    let t = (true, 1, 'c');
    // Destructive
    let (a, b, c) = t; // a is the first value of the tuple t -> a = true, b = 1, c = 'c'

    // ignore with _
    let (_, b, _) = t; // don't need a and c

    // Empty tuple - unit type
    let t = (); // purpose is to return something w/o any data

    // Nested tuple
    let nested = ((3.14, 'a'), (true, 1u32, 'b'), ());
    println!("Nested Tuple - Tuple1: ({}, {})", nested.0.0, nested.0.1);
    println!("Nested Tuple - Tuple2: ({}, {}, {})", nested.1.0, nested.1.1, nested.1.2);
    println!("Nested Tuple - Tuple3: {:?}", nested.2);
}