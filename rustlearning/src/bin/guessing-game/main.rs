use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("🎮 Welcome to the GUESSING GAME 🎮");

    let secret_number = rand::rng().random_range(1..=100);

    internal_loop(secret_number);
}

fn internal_loop(secret_number: u32) {
    loop {
        println!("Please insert your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Yout guess is {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small 😭!!"),
            Ordering::Greater => println!("Too big 😭 !!"),
            Ordering::Equal => {
                println!("You win 😉!!");
                break;
            }
        }
    }
}
