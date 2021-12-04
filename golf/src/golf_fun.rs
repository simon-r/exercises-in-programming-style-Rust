use regex::Regex;
use std::collections::BTreeMap;
use std::collections::{HashMap, HashSet};
use std::fs;

pub fn golf_test() {
    let file_name = String::from("data/text.txt");
    let stop_w_file = String::from("data/stop_words.txt");

    println!(
        "{}",
        Regex::new(r"[\W_]+")
            .unwrap()
            .replace_all(
                &fs::read_to_string(file_name)
                    .expect("Something went wrong reading the file")
                    .to_lowercase(),
                " ",
            )
            .to_string()
            .split_whitespace()
            .fold(
                (
                    fs::read_to_string(&stop_w_file)
                        .expect("Something went wrong reading the stop word file")
                        .split(",")
                        .fold(HashSet::new(), |mut hs, el| {
                            hs.insert(String::from(el));
                            hs
                        }),
                    HashMap::new() as HashMap<String, i32>,
                ),
                |mut hmp, el| {
                    if !hmp.0.contains(el) {
                        *hmp.1.entry(el.to_string()).or_insert(0) += 1
                    };
                    hmp
                },
            )
            .1
            .into_iter()
            .fold(BTreeMap::new(), |mut bt, pair| {
                if pair.1 > 4 {
                    bt.insert((pair.1, pair.0), 0);
                };
                bt
            })
            .iter()
            .rev()
            .fold(String::new(), |mut ss, pair| {
                let k = pair.0;
                ss.push_str(&format!("{} - {}\n", k.1, k.0)[..]);
                ss
            })
    );
}
