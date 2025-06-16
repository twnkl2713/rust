#![allow(unused)]

use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    // Iterator adaptor
    // map, filter, collect, zip, fold
    let vals: Vec<u32> = vec![1, 2, 3];
    let mut data: Vec<u32> = Vec::new();
    for v in vals {
        if v <= 2 {
            data.push(v * 2);
        }        
    }
    let vals: Vec<u32> = vec![1, 2, 3];
    let data: Vec<u32> = vals
        .into_iter()
        .filter(|x| *x <= 2)
        .map(|x| 2 * x)
        .collect();
    // let data: HashSet<u32> = vals.iter().map(|x| 2 * x).collect();
    println!("filter and then map: {:?}", data);

    let keys: Vec<String> = vec!["a", "b", "c", "d"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let vals: Vec<u32> = vec![1, 2, 3];
    let zipped: Vec<(String, u32)> = keys
        .into_iter()
        .zip(vals.into_iter())
        .collect();

    println!("zipped {:?}", zipped);

    let vals: Vec<u32> = vec![1, 2, 3];
    let sum = vals.iter().fold(0, |acc, x| acc + x);
    println!("sum = {sum}");
}
