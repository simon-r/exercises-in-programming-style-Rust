use regex::Regex;
use std::collections::{HashMap, HashSet, LinkedList};
use std::fs;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

macro_rules! make_arc_mutex {
    ($value:expr) => {
        Arc::new(Mutex::new($value))
    };
}

pub fn dataspaces_test(file_name: &String, file_stop_w: &String) {
    let word_space = make_arc_mutex!(LinkedList::<String>::new());
    let freq_space = make_arc_mutex!(HashMap::<String, i32>::new());

    let stop_words = Arc::new(
        fs::read_to_string(file_stop_w)
            .expect("something went wrong in reading stop words")
            .split(",")
            .fold(HashSet::new(), |mut hs, el| {
                hs.insert(String::from(el));
                hs
            }),
    );

    let data = fs::read_to_string(file_name).expect("something went wrong in reading data");

    Regex::new(r"[\W_]+")
        .unwrap()
        .replace_all(&data.to_lowercase(), " ")
        .to_string()
        .split_whitespace()
        .for_each(|el| word_space.lock().unwrap().push_back(String::from(el)));

    let mut jj = LinkedList::<JoinHandle<()>>::new();

    let threads_nr = 20;

    for _ in 0..threads_nr {
        let freq_space_clone = freq_space.clone();
        let word_space_clone = word_space.clone();
        let stop_words_clone = stop_words.clone();

        let join = thread::spawn(move || loop {
            let word = match word_space_clone.lock().unwrap().pop_front() {
                Some(str) => str,
                None => {
                    break;
                }
            };

            if stop_words_clone.contains(&word) || word.len() < 2 {
                continue;
            }

            let mut f = freq_space_clone.lock().unwrap();

            match f.get(&word).cloned() {
                Some(v) => {
                    f.insert(word, v + 1);
                }
                None => {
                    f.insert(word, 1);
                }
            };
            drop(f);
        });

        jj.push_back(join);
    }

    for j in jj {
        let _ = j.join();
    }

    let mut res = Vec::<(String, i32)>::new();

    for f in freq_space.lock().unwrap().iter() {
        if *f.1 > 5 && f.0.len() > 2 {
            res.push((f.0.clone(), f.1.clone()));
        }
    }

    res.sort_by_key(|v| -v.1);

    for v in res {
        println!("{} {}", v.0, v.1);
    }
}
