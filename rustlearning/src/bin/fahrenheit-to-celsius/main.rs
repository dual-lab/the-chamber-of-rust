use std::io;

const F : f64 = 32.0;
const M : f64 = 5.0 / 9.0 ;

fn main() {
    println!("Welcome to fahrenheit to celsius converter");

    let fahr: f64 = loop {
        println!("Fahrenheit");
        let mut fahr = String::new();

        io::stdin()
            .read_line(&mut fahr)
            .expect("Failed to read line");

         match fahr.trim().parse(){
            Ok(num) => break num,
            Err(_) => continue,
        };

    };

    let celsius = internal_converter(fahr);

    println!("{celsius}");
}

fn internal_converter(fahr: f64) -> f64 {
    (fahr - F) * M
}
