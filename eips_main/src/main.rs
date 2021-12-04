use std::io::Write;

use kick_forward;
use mirror;
use pipe;

fn prompt(name: &str) -> String {
    let mut line = String::new();
    print!("{}", name);
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Error: Could not read a line");

    return line.trim().to_string();
}

fn main() {
    println!("Hello, From: Exercises in programming style Rust");

    println!("");

    println!("0. Exit");
    println!("1. Mirror");
    println!("2. Kick Froward");
    println!("3. Pipe");

    println!("");

    loop {
        let select = prompt("Select Style: ");

        if select == "0" {
            break;
        } else if select == "1" {
            mirror::mirror_test();
            break;
        } else if select == "2" {
            kick_forward::kick_forward_test();
            break;
        } else if select == "3" {
            pipe::pipe_test();
            break;
        }
    }
}
