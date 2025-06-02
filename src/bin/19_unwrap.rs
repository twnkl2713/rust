#![allow(unused)]

fn main(){
    // Option
    // let x: Option<i32> git= Some(3);
    // match x{
    //     Some(val) => println!("val = {val}"),
    //     None => panic!("None"),
    // }
    
    // Shortcut to write the code for Option and Result using "Unwrap and Expect"

    // Option
    let x: Option<i32> = Some(3);
    let v = x.unwrap();
    println!("val = {v}");  

    // If x was None:
    // let none_value: Option<i32> = None;
    // let x = none_value.unwrap();  // This will panic!

}