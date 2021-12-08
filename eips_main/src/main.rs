use std::io::Write;

use golf;
use kick_forward;
use mirror;
use pipe;
use the_one;
use things;
use abstract_things;

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

    let file_name = String::from("data/text.txt");
    let file_stop_w = String::from("data/stop_words.txt");

    println!("Hello, From: Exercises in programming style Rust");

    println!("");
    println!("Part II Basic Styles");
    println!("  5. Pipeline");
    println!("  6. Golf");
    println!("");
    println!("Part III Function Composition");
    println!("  7. Infinite Mirror");
    println!("  8. Kick Froward");
    println!("  9. The One");
    println!("");
    println!("Part IV Objects and Object Interaction");
    println!("  10. Things");
    println!("  11. Letterbox");
    println!("  12. Abstract Things");

    println!("");
    println!("0. Exit");
    println!("");

    loop {
        let select = prompt("Select Style: ").parse::<i32>().unwrap_or(9999);

        if select != 9999 {
            println!("");
        }

        if select == 0 {
            break;
        } else if select == 7 {
            mirror::mirror_test();
            break;
        } else if select == 8 {
            kick_forward::kick_forward_test();
            break;
        } else if select == 5 {
            pipe::pipe_test();
            break;
        } else if select == 6 {
            golf::golf_test();
            break;
        } else if select == 9 {
            the_one::the_one_test();
            break;
        } else if select == 10 {
            things::things_test();
            break;
        } else if select == 11 {
            letterbox::letterbox_test(&file_name, &file_stop_w);
            break;
        }  else if select == 12 {
            abstract_things::abstract_things_test(&file_name, &file_stop_w);
            break;
        } 
        else {
            println!("Invalid style ... try again!");
        }
    }
}
