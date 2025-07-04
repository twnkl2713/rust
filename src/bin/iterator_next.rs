fn main() {
    let mut nums = vec![1, 2, 3];
    
    let mut nums_iter = nums.iter_mut();

    // Consumes one item at a time from the mutable iterator
    let first_num: Option<&mut i32> = nums_iter.next();
    let second_num: Option<&mut i32> = nums_iter.next();
    let third_num: Option<&mut i32> = nums_iter.next();

    // At this point, nums_iter is exhausted (all 3 elements consumed), so this loop won't run
    while let Some(val) = nums_iter.next() {
        println!("{}", val);
    }
    println!("{:?}", nums);
}
