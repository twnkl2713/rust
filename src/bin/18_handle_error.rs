#![allow(unused)]

// fn divide(x: u32, y: u32) -> u32 {
//     x / y
// }
#[derive(Debug)]
enum MathError {
    DivByZero,
    Other,
}
fn div(x: u32, y: u32) -> Result<u32, MathError> {
    if y == 0 {
        return Err(MathError::DivByZero);
    }
    Ok(x / y)
}
fn main(){
    // Error
    // panice!("crash");

    // Option or Result
    let arr = [1, 2, 3];
    // arr[9]; -> error
    // Option<&i32> = Some(&i32) | None
    let x: Option<&i32> = arr.get(1);
    match x {
        Some(val) => println!("val = {val}"),
        None => println!("None"),
    }
    let x = 1;
    let y = 0;
    // Result<u32, MathError>
    let z = div(x, y);
    match z {
        Ok(val) => println!("div = {val}"),
        Err(err) => println!("err = {:?}", err),
    }
}