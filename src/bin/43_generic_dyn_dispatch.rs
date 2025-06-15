#![allow(unused)]

#[derive(Debug)]
struct A;

#[derive(Debug)]
struct B;

trait F {
    fn f(&self);
}

impl F for A {
    fn f(&self) {
        println!("{:?}", self)
    }
}

impl F for B {
    fn f(&self) {
        println!("{:?}", self)
    }
}

// static dispatch
// T must resolve to a concrete type at compile time
fn static_dispatch<T: F>(t: &T) {
    t.f();
}

fn dyn_dispatch(t: &dyn F) {
    t.f();
}

// diff b/w &dyn F and Box<dyn F>
// &dyn F - borrow
// Box<dyn F> takes ownership
fn dyn_dispatch_box(t: Box<dyn F>) {
    t.f();
}

fn main() {
    let obj = A;
    static_dispatch(&obj);
    let obj = B;
    static_dispatch(&obj);

    let input = "A";

    let obj: &dyn F = match input {
        "A" => &A,
        "B" => &B,
        _ => panic!(),
    };

    // static_dispatch(obj) -> this won't compile - type must be known at compile time

    dyn_dispatch(obj);

    // Box<dyn T>
    // Allocates memory on the heap, stores concrete instance
    let obj: Box<dyn F> = match input {
        "A" => Box::new(A),
        "B" => Box::new(B),
        _ => panic!(),
    };

    dyn_dispatch_box(obj);
    // let x = obj; -> this won't compile, dyn_dispatch_box takes ownership
}