use std::io;
use rustlearning::language;

fn main() {
    println!("Translate to PIG-LATIN. Please insert a word");
    internal_loop();
}

fn internal_loop() {
    let mut word = String::new();

    io::stdin()
        .read_line(&mut word)
        .expect("Fail to parse the line");

    let word = word.trim();
    let translated = language::pig_latin::translate(&word);
    println!("{word} -> {translated}")
}
