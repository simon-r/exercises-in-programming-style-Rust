use rayon::prelude::*;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::sync::{Arc, Mutex};

type WordsFreq = Arc<Mutex<HashMap<String, i32>>>;

fn merge_freq(a: WordsFreq, b: WordsFreq) -> WordsFreq {
    let res: WordsFreq = Arc::new(Mutex::new(HashMap::new()));

    for w in a.lock().ok().unwrap().iter() {
        res.lock().ok().unwrap().insert(w.0.clone(), w.1.clone());
    }

    for w in b.lock().ok().unwrap().iter() {
        let ca = match res.lock().ok().unwrap().get(w.0) {
            Some(v) => v.clone(),
            None => 0,
        };

        res.lock().ok().unwrap().insert(w.0.clone(), ca + w.1);
    }

    res
}

pub fn map_reduce_test(file_name: &String, file_stop_w: &String) {
    let stop_words = fs::read_to_string(file_stop_w)
        .expect("something went wrong in reading stop words")
        .split(",")
        .into_iter()
        .fold(HashSet::new(), |mut hs, w| {
            hs.insert(w.to_string());
            hs
        });

    let data = Regex::new(r"[\W_]+")
        .unwrap()
        .replace_all(
            &fs::read_to_string(file_name)
                .expect("something went wrong in reading stop words")
                .to_lowercase(),
            " ",
        )
        .to_string();

    let mut vf = Vec::<WordsFreq>::new();

    for w in data.split_whitespace() {
        if stop_words.contains(w) || w.len() < 3 {
            continue;
        }

        let wf: WordsFreq = Arc::new(Mutex::new(HashMap::new()));
        wf.lock().ok().unwrap().insert(String::from(w), 1);
        vf.push(wf.clone());
    }

    let freq_pr = vf.into_par_iter().reduce(
        || Arc::new(Mutex::new(HashMap::new())),
        |a, b| merge_freq(a, b),
    );

    let mut vf = Vec::<(String, i32)>::new();

    for str_f in freq_pr.lock().ok().unwrap().iter() {
        if *str_f.1 > 4 {
            vf.push((str_f.0.clone(), str_f.1.clone()));
        }
    }

    vf.sort_by_key(|el| -el.1);

    for str_f in vf {
        println!("{} - {}", str_f.0, str_f.1);
    }
}
