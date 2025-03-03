use std::io;

fn main() {
    println!("Welcome to fibonacci n");

    let n: u32 = loop {
        println!("Position ");
        let mut n = String::new();

        io::stdin().read_line(&mut n).expect("Failed to read line");

        match n.trim().parse() {
            Ok(num) => break num,
            Err(_) => continue,
        };
    };

    let value = fib_n(n);

    println!("{value}");
}

fn fib_n(pos: u32) -> u32 {
    let mut n_p: u32 = 0;
    let mut n_c: u32 = 1;
    for _ in 1..pos {
        let n_pp = n_c;
        n_c = n_p + n_c;
        n_p = n_pp;
    }
    if pos == 0 { n_p } else { n_c }
}
