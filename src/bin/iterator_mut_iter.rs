// if u want mutable references to the inner variables and don't want to transfer ownership

fn main() {
    let mut nums = vec![1, 2, 3];

    let num_iter = nums.iter_mut();

    for num in num_iter {
        *num = *num + 1;
    }

    println!("{:?}", nums);
}