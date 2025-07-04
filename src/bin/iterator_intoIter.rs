// if u want to move the variable into the iterator and don't want to use it afterwards
// same as that iterator_for
fn main() {
    let nums = vec![1, 2, 3];
    let iter = nums.into_iter();

    for value in iter {
        println!("{}", value);
    }
}