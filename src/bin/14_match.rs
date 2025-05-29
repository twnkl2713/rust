#![allow(unused)]

enum Friends {
    Shinchan,
    Kazama,
    Masao,
    Nene,
    Bochan,
}
fn main(){
    // let x = 1;
    // if x == 1 {
    //     println!("one");
    // } else if x == 2 {
    //     println!("two");
    // } else if x == 3 {
    //     println!("three");
    // } else {
    //     println!("other");
    // }
    // the above same thing can be written using match

    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other"),
    }
    // multiple cases
    match x {
        1 | 2 | 3 => println!("1 or 2 or 3"),
        _ => println!("other"),
    }
    // range
    match x {
        1..=10 => println!("between 1 and 10"),
        _ => println!("other"),
    }
    // the above one can also be used as
    match x {
        i @ 1..=10 => println!("matched {i}"),
        _ => println!("other"),
    }
    // return value
    let friend = Friends::Kazama;
    let friend = match friend {
        Friends::Shinchan => "Action Kamen",
        Friends::Kazama => "English Tution",
        Friends::Masao => "Ichan",
        Friends::Nene => "Office-office",
        Friends::Bochan => "Patthar",
        _ => "?",
    };
    println!("Shinchan characters {friend}");
    // Option
    let x = Some(1);
    match x {
        Some(v) => println!("Some {v}"),
        None => println!("none"),
    }
    // Result
    let res: Result<i32, &str> = Ok(10);
    match res{
        Ok(val) => println!("Ok {val}"),
        Err(msg) => println!("Err {msg}"),
    }
}