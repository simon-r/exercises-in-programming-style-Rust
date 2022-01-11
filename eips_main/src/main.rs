use std::io::Write;

use abstract_things;
use actors;
use aspects;
use bulletin_board;
use closed_maps;
use constructivist;
use golf;
use hollywood;
use kick_forward;
use lazy_rivers;
use lazy_rivers_mp;
use mirror;
use persistent_tables;
use pipe;
use quarantine;
use the_one;
use things;

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
    println!("  12. Closes Map");
    println!("  13. Abstract Things");
    println!("  14. Hollywood");
    println!("  15. Bulletin Board");
    println!("");
    println!("Part V Reflection and Metaprogramming");
    println!("  18. Bulletin Board");
    println!("");
    println!("Part VI Adversity");
    println!("  20. Constructivist");
    println!("  21. Tantrum");
    println!("  24. Quarantine");
    println!("");
    println!("Part VII Data-Centric");
    println!("  25.  Persistent Tables");
    println!("  27.  Lazy Rivers:  Iterators");
    println!("  271. Lazy Rivers:  Message Passing");
    println!("");
    println!("Part VIII Data-Centric");
    println!("  28.  Actors");
    println!("");
    println!("0. Exit");
    println!("");

    loop {
        let select = prompt("Select Style: ").parse::<i32>().unwrap_or(9999);

        println!("");

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
        } else if select == 12 {
            closed_maps::closed_maps_test(&file_name, &file_stop_w);
            break;
        } else if select == 11 {
            letterbox::letterbox_test(&file_name, &file_stop_w);
            break;
        } else if select == 13 {
            abstract_things::abstract_things_test(&file_name, &file_stop_w);
            break;
        } else if select == 14 {
            hollywood::hollywood_test(&file_name, &file_stop_w);
            break;
        } else if select == 15 {
            bulletin_board::bulletin_board_test(&file_name, &file_stop_w);
            break;
        } else if select == 18 {
            aspects::aspects_test(&file_name, &file_stop_w);
            break;
        } else if select == 20 {
            constructivist::constructivist_test(&file_name, &file_stop_w);
            break;
        } else if select == 21 {
            tantrum::tantrum_test(&file_name, &file_stop_w);
            break;
        } else if select == 24 {
            quarantine::quarantine_test(&file_name, &file_stop_w);
            break;
        } else if select == 25 {
            persistent_tables::persistent_tables_test(&file_name, &file_stop_w);
            break;
        } else if select == 27 {
            lazy_rivers::lazy_rivers_test(&file_name, &file_stop_w);
            break;
        } else if select == 271 {
            lazy_rivers_mp::lazy_rivers_mp_test(&file_name, &file_stop_w);
            break;
        } else if select == 28 {
            actors::actors_test(&file_name, &file_stop_w);
            break;
        } else {
            println!("Invalid style ... try again!");
        }
    }
}
