use rand::RngExt;
use std::{thread, time::Duration};

pub fn run() {
    loop {
        let x: u32 = rand::random();
        println!("Random number: {x}");

        let number = rand::rng().random_range(100..200);
        println!("Random number between 100 and 200 : {number}");

        println!("---------------------------------------------------------");

        thread::sleep(Duration::from_secs(2));
    }
}
