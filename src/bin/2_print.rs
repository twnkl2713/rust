#![allow(unused)]
#[derive(Debug)]
// structure
struct Lang{
    language: String,
    version: String
}
fn main(){
    let lang = "rust";
    println!("hello {}", lang);
    println!("hello {} {}", lang, lang);
    println!("hello {lang}");

    let x = 2;
    println!("{0} x {0} = {1}", x, x * x);

    let lang = Lang{
        language: "rust".to_string(),
        version: "1.83".to_string(),
    };

    let name = "John";
    let age = 30;
    println!("{} is {} years old.", name, age);

    // to change the value of the variable, u must use the "mut" keyword
    let mut x = 5;
    println!("Before: {}", x);
    x = 10;
    println!("After: {}", x);

    // to print the lang structure
    println!("{:?}", lang);
    println!("{:#?}", lang); 
}