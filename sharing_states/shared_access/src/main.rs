#![allow(unused)]
use std::sync::Mutex;
use std::thread;
use std::sync::Arc;
fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            let mut val = counter.lock().unwrap();
            *val+=1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join();
    }
    println!("{counter:?}");
}
