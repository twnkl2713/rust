// if u want immutable references to the inner variables and don't want to transfer ownership

fn main() {
    let nums = vec![1, 2, 3];
    
    let nums_iter = nums.iter(); // borrowing

    for num in nums_iter {
        println!("Got: {num}");
    }

    println!("{:?}", nums);
}

// you can't mutate the variables since we have an immutable reference to the internal elements