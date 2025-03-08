use std::io;

pub fn line(message: &str) -> String {
    println!("{message}");
    let mut line = String::new();
    io::stdin()
    .read_line(&mut line)
        .expect("Error reading line");

    let line = line.trim();
    line.to_string()
}
