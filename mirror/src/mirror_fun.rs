use regex::Regex;
use std::collections::{HashMap, HashSet, LinkedList};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn read_file(file_name: &String) -> LinkedList<String> {
    let text = fs::read_to_string(file_name).expect("Something went wrong reading the file");

    let re = Regex::new(r"[\W_]+").unwrap();
    let content = re.replace_all(&text[..], " ");

    content
        .to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .map(|word| String::from(word))
        .collect()
}

pub fn read_stop_words(file_name: &String) -> HashSet<String> {
    let stop_words: HashSet<String> = BufReader::new(File::open(file_name).unwrap())
        .split(b',')
        .map(|b| String::from_utf8_lossy(&b.unwrap()).into())
        .collect();
    stop_words
}

pub fn count(
    stop_words: &HashSet<String>,
    word_list: &mut LinkedList<String>,
    word_freqs: &mut HashMap<String, i32>,
) {
    if word_list.len() == 0 {
        return;
    } else {
        let word = word_list.pop_front().unwrap();

        if !stop_words.contains(&word) && word.len() > 1 {
            *(word_freqs.entry(word).or_insert(0)) += 1;
        }

        count(stop_words, word_list, word_freqs);
    }
}

pub fn print_freqs(word_freqs: &HashMap<String, i32>) -> String {
    let mut vp = word_freqs
        .into_iter()
        .fold(Vec::<(String, i32)>::new(), |mut v, el| {
            v.push((String::from(el.0), *el.1));
            v
        });

    vp.sort_by_key(|k| -k.1);

    vp.iter()
        .filter(|p| p.1 > 4)
        .fold(String::new(), |mut st, p| {
            st.push_str(&format!("{}   -  {}\n", p.0, p.1));
            st
        })
}

pub fn mirror_test() {

    let file_name = String::from("data/text.txt");

    let stop_w = read_stop_words(&String::from("data/stop_words.txt"));
    let mut words = read_file(&file_name);

    let mut word_freqs: HashMap<String, i32> = HashMap::new();
    count(&stop_w, &mut words, &mut word_freqs);

    let res_mirror = print_freqs(&word_freqs);
    println!("{}", res_mirror);
}
