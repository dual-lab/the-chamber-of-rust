use std::io;

use rustlearning::sequence::{find_median, find_mode};

fn main() {
    println!("Please insert a sequence of numbers");
    internal_loop();
}

fn internal_loop() {
    let mut seq: Vec<i32> = Vec::new();
    loop {
        let mut z = String::new();
        io::stdin().read_line(&mut z).expect("Failed to read line");

        match z.trim().parse() {
            Ok(num) => seq.push(num),
            Err(_) => break,
        };
    }

    match find_median(&seq) {
        Some(m) => println!("Median of seq is {m}"),
        None => println!("No median found"),
    }
    for (key, value) in find_mode(&seq) {
        println!("{key} => {value}")
    }
}
