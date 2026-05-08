#![allow(unused)]
use core::time;
use std::thread;
use std::sync::mpsc;



fn main() {
    let (tx,rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move ||{
        let messages = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("the "),
            String::from("thread"),
        ];
        for msg in messages {
            tx1.send(msg);
            thread::sleep(time::Duration::from_secs(1));
        }
    });
    thread::spawn(move ||{
        let messages = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for msg in messages {
            tx.send(msg);
            thread::sleep(time::Duration::from_secs(2));
        }
    });

    for received in rx {
        println!("{}",received);
    }

}
