#![allow(unused)]

fn main(){
    let x = -123;
    // x += 1; -> This will not compile as it is immutable(value can't be changed)
    
    let mut y = 123;
    y += 1;
    println!("{}", y);

    let z = -123;

    const NUM: u32 = 1;

    let x = -1;
    let x = true;

    println!("{}", x);
    let v: Vec<_> = vec![1, 2, 3];

    // w3schools

    let my_num = 5; //integer
    let my_double = 5.99; // double
    let my_letter = 'D'; // character
    let my_bool = true; // boolean
    let my_text = "Hello"; // string

    let price = 19.99;
    println!("Price is: ${}", price);
}