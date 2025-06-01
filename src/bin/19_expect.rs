#![allow(unused)]

// Result
// let x = 1;
// let y = 1;
// let z: Result<u32, String> = Ok(x / y);
// match z {
//     Ok(val) => println!("div = {val}"),
//     Err(err) => println!("err = {:?}", err),
// }

fn main() {
    let result: Result<i32, &str> = Ok(10);

    let value = result.unwrap();
    println!("Value: {}", value);

    // If it was an error:
    // let error_result: Result<i32, &str> = Err("Oops");
    // let value = error_result.unwrap(); // Panic with a generic message
}
