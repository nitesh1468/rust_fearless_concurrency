#![allow(unused)]
use std::time::Duration;
use std::thread;
fn main() {
    let handle = thread::spawn(||{
        for idx in 0..10 {
            println!("hi number {idx} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for idx in 0..5 {
        println!("hi number {idx} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
}
