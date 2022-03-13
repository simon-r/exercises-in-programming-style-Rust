use rayon::prelude::*;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

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

pub fn double_map_reduce_test(file_name: &String, file_stop_w: &String) {
    let stop_words = Arc::new(Mutex::new(
        fs::read_to_string(file_stop_w)
            .expect("something went wrong in reading stop words")
            .split(",")
            .into_iter()
            .fold(HashSet::new(), |mut hs, w| {
                hs.insert(w.to_string());
                hs
            }),
    ));

    let threads_nr = 10;
    let mut join_vec = Vec::<JoinHandle<()>>::new();
    join_vec.reserve(threads_nr);

    let vf_g = Arc::new(Mutex::new(Vec::<WordsFreq>::new()));
    let nlines = 100;

    let file = File::open(file_name).ok().unwrap();
    let reader = Arc::new(Mutex::new(BufReader::new(file)));

    for _i in 0..threads_nr {
        let reader_clone = reader.clone();
        let stop_words_clone = stop_words.clone();
        let vf_g_clone = vf_g.clone();

        let j = thread::spawn(move || loop {
            let mut buf = String::new();

            for _l in 0..nlines {
                let _line_sz: i32 = match reader_clone.lock().ok().unwrap().read_line(&mut buf) {
                    Ok(v) => v as i32,
                    Err(_v) => -1 as i32,
                };
            }

            let mut vf = Vec::<WordsFreq>::new();

            let data = Regex::new(r"[\W_]+")
                .unwrap()
                .replace_all(&buf.to_lowercase(), " ")
                .to_string();

            for w in data.split_whitespace() {
                if stop_words_clone.lock().ok().unwrap().contains(w) || w.len() < 3 {
                    continue;
                }

                let wf: WordsFreq = Arc::new(Mutex::new(HashMap::new()));
                wf.lock().ok().unwrap().insert(String::from(w), 1);
                vf.push(wf.clone());
            }

            match vf.into_iter().reduce(|a, b| merge_freq(a, b)) {
                Some(ww) => {
                    vf_g_clone.lock().ok().unwrap().push(ww.clone());
                }
                None => {
                    return;
                }
            };
        });
        join_vec.push(j);
    }

    for j in join_vec {
        let _ = j.join();
    }

    let vf = vf_g.lock().ok().unwrap().clone();

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
