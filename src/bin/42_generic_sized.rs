#![allow(unused)]

// Sized and ?Sized - indicates that a type's size is known or not at the compile time

fn f<T: Sized>(x: T) {}
// this will not compile - T must be a pointer

fn g<T: ?Sized>(x: &T) {}

fn main() {
    // primitive types
    let i: i32 = 1;
    let x: f64 = 1.0;
    let b: bool = true;

    // structs and enums with sized fields
    struct S {
        i: i32,
        j: i32,
    };

    let s = S{i: 1, j: 1};

    // fixed size array
    let arr: [i32; 4] = [0; 4];

    f(i);
    f(s);
    f(arr);
    f(&arr);
    f("rust");

    // str and slices
    let s: &str = "hello";
    let slice: &[i32] = &[1, 2, 3];

    g(s);
    g(slice);

    // Trait objects
    let v: Box<dyn A> = Box::new(1u32);
    d(v);

    let v: Box<dyn A> = Box::new(1u32);
    g(&v);
}

trait A {}

impl A for u32 {}

fn d(x: Box<dyn A>) {}