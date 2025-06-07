#![allow(unused)]

use std::collections::HashSet;

fn main() {
    let mut set: HashSet<u32> = HashSet::new();

    let inserted: bool = set.insert(1);
    println!("Inserted: {inserted}");

    let inserted: bool = set.insert(1);
    println!("Inserted: {inserted}");

    let contains: bool = set.contains(&1);
    println!("Contains 1?: {contains}");

    let contains: bool = set.contains(&2);
    println!("Contains 2?: {contains}");

    let contains: bool = set.contains(&2);
    println!("contains 3?: {contains}");
}