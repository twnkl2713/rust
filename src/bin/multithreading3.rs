use std::thread;

fn main() {
    let x = 1;
    {
        let v = vec![1, 2, 3];
        thread::spawn(move || {
            println!("{:?}", v);
        })
        .join()
        .unwrap(); // wait for thread to finish
    }
    println!("Outside thread: {}", x);
}

// with move - takes ownership of variables, here giving the whole 'v' to the thread. it owns it now.
// without move - borrows outer variables, here it might need v from outside.