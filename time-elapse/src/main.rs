#![allow(dead_code, unused)]
use std::thread::{sleep, spawn};
use std::time::{Duration, Instant};

fn main() {
    let instant: Instant = Instant::now();
    let deadline = Duration::from_secs(5);
    let handle = spawn(move || loop {
        if instant.elapsed() > deadline {
            break;
        }
        sleep(Duration::from_millis(500));
        println!("loop...");
    });

    handle.join().unwrap();
}
