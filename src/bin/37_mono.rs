#![allow(unused)]

// monomorphization with generics

// generic struct
struct Point<T> {
    x: T,
    y: T,
}

//  non generic versions
struct PointU32 {
    x: u32,
    y: u32,
}

struct PointI32 {
    x: i32,
    y: i32,
}

// generic function to get x
fn get_x<T>(p: &Point<T>) -> &T {
    &p.x
}

// non generic versions of the function
fn get_x_u32(p: &PointU32) -> u32 {
    p.x
}

fn get_x_i32(p: &PointI32) -> i32 {
    p.x
}

fn main() {
    // using generic Point<T>
    let p0: Point<u32> = Point { x: 10, y: 20 };
    let p1: Point<i32> = Point { x: -10, y: -20 };

    println!("Generic u32 x: {}", get_x(&p0));
    println!("Generic i32 x: {}", get_x(&p1));

    // using non generic PointU32 and PointI32
    let p0 = PointU32 { x: 30, y: 40 };
    let p1 = PointI32 { x: -30, y: -40 };

    println!("Non-generic u32 x: {}", get_x_u32(&p0));
    println!("Non-generic i32 x: {}", get_x_i32(&p1));
}
