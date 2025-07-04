fn main() {
    let v1: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let iter = v1.iter();
    let new_iter = v1.iter();
    let iter2 = iter.filter(|x| *x % 2 == 0); // filters out even
    let iter3 = new_iter.filter(|x| *x % 2 == 1).map(|x| x * 2); // filters out odd, then returns their double
    
    println!("iter2: ");
    for x in iter2 {
        print!("{} ", x);
    }
    println!("\niter3: ");
    for i in iter3 {
        print!("{} ", i);
    }
    println!("\nv1: ");
    println!("{:?}", v1);
}