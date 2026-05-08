#![allow(unused)]
use std::thread;
use std::sync::mpsc;
fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move ||{
        let val = String::from("Hi");
        tx.send(val).unwrap();
    });
    let reveived = rx.recv().unwrap();
    println!("Got: {}",reveived);
}
