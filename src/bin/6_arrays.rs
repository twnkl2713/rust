#![allow(unused)]

fn main(){
    // Arrays
    let arr= [1, 2, 3];
    println!("arr[2] = {}", arr[2]);

    let mut arr = [1, 2, 3];
    arr[0] = 10;
    println!("The new first number is: {}", arr[0]);

    let arr = [0; 10]; // [0,0,0,...,0] -> 10 zeros
    println!("{:?}", arr); // for printing whole array {:?} is used

    // Slice
    let nums = [-1, 1, -2, 2, -3, 3, -4, 4, -5, 5];

    // First 3 elements
    let s = &nums[0..3]; // [0..3] can also be written as [..3] -> (0, 1, 2)
    println!("First three elements: {:?}", s);

    // Middle 4 elements
    let s = &nums[3..7];
    println!("Middle four elements: {:?}", s);

    // Last 3 elements
    let s = &nums[7..];
    println!("Last three elements: {:?}", s);

    // All elements
    let s = &nums[..];
    println!("All elements: {:?}", s);
}