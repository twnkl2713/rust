#![allow(unused)]

use std::collections::{HashMap, HashSet};

/* 
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
*/

struct Counter {
    pub count: u32,
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count <= 5 {
            Some(self.count)
        }
        else {
            None
        }
    }
}

fn main() {
    // iter - borrows and returns an iterator that returns &T
    // into_iter - takes ownership and returns an iterator that may return T, &T or &mut T
    // iter_mut - returns &mut T

    let mut counter = Counter {count: 0};
    while let Some(c) = counter.next() {
        println!("count {}", c);
    }

    // default implementation of into_iter is implicitly called
    let counter = Counter {count: 0};
    for c in counter {
        println!("count {}", c);
    }

    // iter
    // Use iter to loop multiple times
    let vals: Vec<i32> = vec![1, 2, 3];
    for v in vals.iter() {
        // v has type &i32
        println!("iter: {}", v);
    }
    for v in vals.iter() {
        println!("iter: {}", v);
    }

    // iter_mut
    let mut vals = vec![1, 2, 3];
    for v in vals.iter_mut() {
        *v += 10;
    }
    println!("iter_mut: {:?}", vals);
}