#![allow(unused)]

// Trait bound - specifies constraints on a generic type

trait A {}
trait B {}
trait C {}

impl A for u32 {}
impl B for u32 {}
impl C for u32 {}

impl A for i32 {}
impl B for i32 {} 
impl C for i32 {} 

// Trait bounds
// x must implement A
fn c<T: A>(x: T) {}

// Multiple trait bounds
// x must implement A and B
fn m<T: A + B>(x: T) {}

// Where clause
// x must implement A and B
// y must implement B and C
fn w<T, U>(x: T, y: U)
where
    T: A + B,
    U: B + C,
{
}

// Difference between impl Trait and generic bounds

// x and y can be different types
fn f(x: impl A, y: impl B) {}

// x and y must be the same type
fn g<T: A>(x: T, y: T) {}

// x and y can be different types
fn h<T: A, U: A>(x: T, y: U) {}

fn main() {
    let x: u32 = 1;
    let y: i32 = 1;
    let z: f32 = 1.0;

    c(x);
    // c(z); -> This won't compile -> f32 doesn't implement A
    

    m(x);
    // m(y); -> This won't compile, i32 doesn't implement B
    

    m(y); // compiles after implementing B for i32

    w(x, x);
    // w(x, y); -> This won't compile, i32 doesn't implement B and C
    

    w(y, y); // compiles after implementing B+C for i32

    f(x, x);
    f(y, y);
    f(x, y);

    g(x, x);
    g(y, y);
    // g(x, y); -> This won't compile

    h(x, x);
    h(y, y);
    h(x, y);
}
