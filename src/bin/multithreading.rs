use std::thread;
use std::time::Duration;

fn main() {
    let sum = 0;
    let handle = thread::spawn( || {
        for i in 1..10 {
            println!("Hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(i));
        }
    }); 

    handle.join().unwrap(); // awaiting the thread to foinish before running the iteration on the main thread

    for i in 1..5 {
        println!("Hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}
