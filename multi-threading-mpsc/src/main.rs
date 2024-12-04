#![allow(dead_code, unused)]
use std::sync::mpsc;
use std::thread;

fn main() {
    let (sender, receiver) = mpsc::channel();
    let sender2 = mpsc::Sender::clone(&sender);

    thread::spawn(move || {
        for i in 1..100 {
            sender.send(i).unwrap();
        }
    });

    thread::spawn(move || {
        for i in 100..200 {
            sender2.send(i).unwrap();
        }
    });

    for val in receiver {
        println!("Received from thread: {}", val);
    }
}
