use regex::Regex;
use std::collections::{HashMap};
use std::fs;

pub fn read_file(file_name: &String) -> String {
    let content = fs::read_to_string(file_name).expect("Something went wrong reading the file");
    content
}

pub fn filter_chars_and_normalize(text: &String) -> String {
    let re = Regex::new(r"[\W_]+").unwrap();
    let result = re.replace_all(&text[..], " ");
    String::from(result)
}

pub fn scan_str(text: &String) -> Vec<String> {
    let split = text.split_whitespace();

    let mut v_str: Vec<String> = Vec::new();

    for s in split {
        v_str.push(String::from(s).to_lowercase());
    }

    v_str
}

pub fn remove_stop_words(v_str: &Vec<String>) -> Vec<String> {
    let stop_w = fs::read_to_string("data/stop_words.txt")
        .expect("Something went wrong reading the stopw.txt file");

    let mut stop_w_split: Vec<&str> = stop_w.split(",").collect();
    stop_w_split.sort();

    let mut res_v: Vec<String> = Vec::new();

    for w in v_str.iter() {
        if stop_w_split.binary_search(&&w[..]).is_err() && w.len() > 1 {
            res_v.push(String::from(w));
        }
    }
    res_v
}

pub fn frequencies(word_list: &Vec<String>) -> HashMap<String, i32> {
    let mut freq: HashMap<String, i32> = HashMap::new();

    for w in word_list.iter() {
        let cnt = freq.get(w);
        match cnt {
            Some(v) => {
                let nv = v + 1;
                freq.insert(String::from(w), nv)
            }
            None => freq.insert(String::from(w), 1),
        };
    }
    freq
}

pub fn sort_wf(word_freq: &HashMap<String, i32>) -> Vec<(String, i32)> {
    let mut res_dict: Vec<(String, i32)> = Vec::new();

    for (w, f) in word_freq.into_iter() {
        res_dict.push((String::from(w), *f));
    }

    res_dict.sort_by_key(|k| 0 - k.1);

    res_dict
}

pub fn print_all(word_freq: &Vec<(String, i32)>) -> String {
    let mut str_res = String::from("");

    for (w, f) in word_freq.iter() {
        if *f > 4 {
            str_res.push_str(&format!("{:.<20}{}\n", w, f)[..]);
        }
    }
    str_res
}

pub fn pipe_test() {
    println!(
        "{}",
        print_all(&sort_wf(&frequencies(&remove_stop_words(&scan_str(
            &filter_chars_and_normalize(&read_file(&String::from("data/text.txt"))),
        )))))
    );
}
