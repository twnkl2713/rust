#![allow(unused)]

// Fn, FnMut and FnOnce traits

fn f_fn<T, F: Fn() -> T>(f: F) {
    // can call more than once
    f();
    f();
}

fn f_mut<T, F: FnMut() -> T>(mut f: F) {
    // can call more than once
    f();
    f();
}

fn f_once<T, F: FnOnce() -> T>(f: F) {
    // can't call more than once
    // f();
    f();
}

fn main() {
    let s = "hello".to_string();
    let f = ||println!("fn : {}", s);
    f_fn(f);
    f_fn(f); // can call more than once
    println!("main: {}", s);

    // FnMut
    let mut v = vec![];
    let mut f = || v.push(0);
    f_mut(&mut f);
    f_mut(&mut f); // can call more than once
    println!("main: {:?}", v);

    // FnOnce
    let v = vec![0, 1, 2];
    let f = move || println!("fn once: {:?}", v); // force transfer of v's ownership
    f_once(f);
    // f_once(f); -> can't call more than once
}