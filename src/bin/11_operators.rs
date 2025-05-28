#![allow(unused)]

fn main(){
    // +, -, *, /
    let a = 1;
    let b = 2;
    let c  =3;
    let c = a - b;
    println!("{a} - {b} = {c}");
    let c = a + b;
    println!("{a} + {b} = {c}");
    let c = a / b;
    println!("{a} / {b} = {c}");
    // % (remainder != mod operator)
    // mod -> a % b = r, 0<=r<b (-1 % 2 = 1)
    // remainder -> -1 % 2 = -1
    let x = -1;
    let y = 2;
    let rem = x % y; 
    println!("{a} % {b} = {rem}");

    // Literals
    let a = 1i32;
    let b = 3u64;
    let c = 3.14e3; 
    let d = 1_000_000_000u32;

    // Boolean
    let a = true && false;
    let a = true || false;
    let a = !true;

    // Bitwise
    // 101
    let a = 5;
    // 110
    let b = 7;
    println!("a & b = {:03b}", a & b);
    println!("a | b = {:03b}", a | b);
    println!("a ^ b = {:03b}", a ^ b);
    println!("!a = {:03b}", !a);
    println!("2 << 5 = {}", 2u32 << 5); 
    println!("10 >> 2 = {}", 10u32 >> 2); 
}