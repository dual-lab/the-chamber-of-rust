use std::{env, process};
use rustlearning::minigrep::Config;
/// Recap rust feature Struct + generic + Std IO + Lifetime
///
///     Binary: mingrep
/// Descriptio: Simple grep cli command, to find a string in a file
///      Usage: mingrep <string-to-serach> <path-to-file> <-i>
///
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Programm exit with error: {err}");
        process::exit(1);
    });

    if let Err(e) = rustlearning::minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

