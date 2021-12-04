use std::io::Write;

use golf;
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
    println!("Part II Basic Styles");
    println!("5. Pipeline");
    println!("6. Golf");
    println!("");
    println!("Part III Function Composition");
    println!("7. Infinite Mirror");
    println!("8. Kick Froward");

    println!("");
    println!("0. Exit");
    println!("");

    loop {
        let select = prompt("Select Style: ");

        if select == "0" {
            break;
        } else if select == "7" {
            mirror::mirror_test();
            break;
        } else if select == "8" {
            kick_forward::kick_forward_test();
            break;
        } else if select == "5" {
            pipe::pipe_test();
            break;
        } else if select == "6" {
            golf::golf_test();
            break;
        }
    }
}
