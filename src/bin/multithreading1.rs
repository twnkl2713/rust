use std::thread;

fn main() {
    // start thread 1
    let handle1 = thread::spawn(|| {
        let mut c = 0;
        for i in 0..50000000 {
            for j in 0..50000000 {
                c = c + 1;
                c = c - 1;
            }
        }
        println!("Thread 1 finished");
    });

    // start thread 2
    let handle2 = thread::spawn(|| {
        let mut c = 0;
        for i in 0..5 {
            for j in 0..5 {
                c = c + 1;
                c = c - 1;
            }
        }
        println!("Thread 2 finished");
    });

    // main thread starts working
    let mut c = 0;
    for i in 0..5 {
        for j in 0..5 {
            c = c + 1;
            c = c - 1;
            println!("hi from spawned thread {}", i);
        }
    }
    println!("Main thread finished");

    handle1.join().unwrap(); // wait for thread 1 to finish
    handle2.join().unwrap(); // wait for thread 2 to finish
}

// without .join(), program might finish before threads complete 
// order of execution is not fixed, it's decided by the OS
// .join() waits for the thread to finish execution
