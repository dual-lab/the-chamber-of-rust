use std::collections::HashMap;

use rustlearning::input;

fn main() {
    println!("Departement management APP");
    internal_loop();
}

fn internal_loop() {
    let mut database: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        print_instruction();
        let cmd = input::line("Select one");
        let cmd = &cmd[..];
        match cmd {
            "A" => add_to_department(&mut database),
            "L" => find_by_department(&database),
            "Q" => break,
            _ => continue,
        }
    }
}

fn add_to_department(database: &mut HashMap<String, Vec<String>>) {
    let department = input::line("Insert the department");
    let employe = input::line("Insert employe name");

    database
        .entry(department)
        .or_insert(Vec::new())
        .push(employe);
}

fn find_by_department(database: &HashMap<String, Vec<String>>) {
    let department = input::line("Insert a department");
    match database.get(&department) {
        Some(e) => {
            for name in e {
                println!("{name}")
            }
        }
        None => println!("No employes in {department}"),
    }
}

fn print_instruction() {
    println!("A: add to Departement");
    println!("L: list employes of a Departement");
    println!("Q: quit the app");
}
