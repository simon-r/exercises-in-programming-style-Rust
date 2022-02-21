use regex::Regex;
use std::borrow::BorrowMut;
use std::collections::{BTreeMap, HashMap, HashSet, LinkedList};
use std::fs;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

macro_rules! make_arc_mutex {
    ($value:expr) => {
        Arc::new(Mutex::new($value))
    };
}

pub fn dataspaces_test(file_name: &String, file_stop_w: &String) {
    let mut word_space = make_arc_mutex!(LinkedList::<String>::new());
    let mut freq_space = make_arc_mutex!(HashMap::<String, i32>::new());

    let stop_words = Arc::new(fs::read_to_string(file_stop_w)
        .expect("something went wrong in reading stop words")
        .split(",")
        .fold(HashSet::new(), |mut hs, el| {
            hs.insert(String::from(el));
            hs
        }));

    let data = fs::read_to_string(file_name).expect("something went wrong in reading data");

    Regex::new(r"[\W_]+")
        .unwrap()
        .replace_all(&data.to_lowercase(), " ")
        .to_string()
        .split_whitespace()
        .for_each(|el| word_space.lock().unwrap().push_back(String::from(el)));

    let mut jj = LinkedList::<JoinHandle<()>>::new();

    for n in 0..20 {
        let freq_space_clone = freq_space.clone();
        let word_space_clone = word_space.clone();
        let stop_words_clone = stop_words.clone();

        let join = thread::spawn(move || loop {
            let word = match word_space_clone.lock().unwrap().pop_front() {
                Some(str) => str,
                None => {
                    break;
                    String::from("")
                }
            };

            if stop_words_clone.contains(&word) || word.len() < 2 {
                continue;
            }

            let f = freq_space_clone.lock().unwrap().get(&word).cloned();

            match f {
                Some(v) => {
                    freq_space_clone.lock().unwrap().insert(word, v + 1);
                }
                None => {
                    freq_space_clone.lock().unwrap().insert(word, 1);
                }
            };
        });

        jj.push_back(join);
    }

    for j in jj {
        let _ = j.join();
    }

    for f in freq_space.lock().unwrap().iter() {
        println!("{} {}", f.0, f.1);
    }
}
