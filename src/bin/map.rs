// iterator adaptor - methods defined on the iterator trait that don't consume the iterator. instead, they produce different iterators by changing some aspect of the original iterator

fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let iter = v1.iter();
    let iter2 = iter.map(|x| x + 1);
    for x in iter2 {
        println!("{}", x); // 2 3 4
    }
    println!("{:?}", v1);
}