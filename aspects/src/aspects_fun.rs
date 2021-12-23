use regex::Regex;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::rc::Rc;
use std::time::Instant;

fn chrono_fun<A, R, F>(fun: F, message: &Rc<RefCell<String>>) -> impl Fn(A) -> R
where
    F: Fn(A) -> R,
{
    let message_clone = message.clone();

    move |x| {
        let now = Instant::now();
        let res = fun(x);
        let ms = format!(" {} micro sec.", now.elapsed().as_micros());
        message_clone.clone().borrow_mut().clone_from(&ms);
        res
    }
}

fn read_and_filter_words(in_files: (&String, &String)) -> Vec<String> {
    let stop_w_file = in_files.1;
    let stop_words = fs::read_to_string(stop_w_file)
        .expect("something went wrong in reading stop words")
        .split(",")
        .into_iter()
        .map(|el| String::from(el))
        .collect::<HashSet<String>>();

    let path_to_file = in_files.0;

    Regex::new(r"[\W_]+")
        .unwrap()
        .replace_all(
            &fs::read_to_string(path_to_file)
                .expect("something went wrong in reading stop words")
                .to_lowercase(),
            " ",
        )
        .to_string()
        .split_whitespace()
        .map(|el| String::from(el))
        .filter(|el| !stop_words.contains(el) && el.len() > 1)
        .collect::<Vec<String>>()
}

fn frequencies_to_string(vec_str: &Vec<String>) -> String {
    let mut vs = vec_str
        .iter()
        .fold(HashMap::new(), |mut hm, el| {
            *hm.entry(el.clone()).or_insert(0) += 1;
            hm
        })
        .into_iter()
        .map(|el| el)
        .collect::<Vec<_>>();
    vs.sort_by_key(|el| -el.1);

    vs.iter().fold(String::new(), |mut st, el| {
        if el.1 > 4 {
            st.push_str(&format!("{} - {}\n", el.0, el.1));
        }
        st
    })
}

//////////////////////////////////////////////////////////////////
pub fn aspects_test(file_name: &String, file_stop_w: &String) {
    let msg_1 = Rc::new(RefCell::new(String::new()));

    let chrono_read_and_filter_words = chrono_fun(read_and_filter_words, &msg_1);
    let vec_str = chrono_read_and_filter_words((file_name, file_stop_w));

    let msg_2 = Rc::new(RefCell::new(String::new()));
    let chrono_frequencies_to_string = chrono_fun(frequencies_to_string, &msg_2);
    let res = chrono_frequencies_to_string(&vec_str);

    println!("{}\n", res);

    println!("Duration: read_and_filter_words {}", msg_1.clone().borrow());
    println!("Duration: frequencies_to_string {}\n", msg_2.clone().borrow());
}
