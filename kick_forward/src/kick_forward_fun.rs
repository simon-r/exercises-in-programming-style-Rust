use regex::Regex;
use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::fs;

pub type ArgFun = Box<dyn Fn(&String) -> ()>;
pub type ArgFunVec = Box<dyn Fn(&Vec<String>) -> ()>;
pub type ArgFunHash = Box<dyn Fn(&HashMap<String, i32>) -> ()>;
pub type ArgFunVecPairs = Box<dyn Fn(&Vec<(String, i32)>) -> ()>;

pub fn read_file_kf(file_name: &String, f: &dyn Any) {
    if f.is::<ArgFun>() {
        let text = fs::read_to_string(file_name).expect("Something went wrong reading the file");
        let fun = f.downcast_ref::<ArgFun>().unwrap();

        fun(&text);
    } else {
        println!("{}", "fail - read_file_kf");
    }
}

pub fn filter_chars_and_normalize_kf(text: &String, f: &dyn Any) {
    match f.downcast_ref::<ArgFun>() {
        Some(fun) => {
            let norm_text = Regex::new(r"[\W_]+")
                .unwrap()
                .replace_all(&text.to_lowercase(), " ")
                .to_string();

            fun(&norm_text);
        }
        None => println!("Fail- filter_chars_and_normalize\n"),
    }
}

pub fn scan_string_kf(text: &String, f: &dyn Any) {
    match f.downcast_ref::<ArgFunVec>() {
        Some(fun) => {
            let v: Vec<String> =
                text.split_whitespace()
                    .filter(|s| s.len() > 1)
                    .fold(Vec::new(), |mut v, el| {
                        v.push(el.to_string());
                        v
                    });

            fun(&v);
        }
        None => println!("Fail- scan_str\n"),
    }
}

pub fn remove_stop_words_kf(v_str: &Vec<String>, f: &dyn Any) {
    let stop_w: HashSet<String> = fs::read_to_string("data/stop_words.txt")
        .expect("Something went wrong reading the stopw.txt file")
        .split(",")
        .fold(HashSet::new(), |mut hs, el| {
            hs.insert(String::from(el));
            hs
        });

    match f.downcast_ref::<ArgFunVec>() {
        Some(fun) => {
            let v_str_fil: Vec<String> = v_str.iter().filter(|el| !stop_w.contains(&el[..])).fold(
                Vec::new(),
                |mut vs, el| {
                    vs.push(String::from(el));
                    vs
                },
            );

            fun(&v_str_fil);
        }
        None => println!("Fail- scan_str\n"),
    }
}

pub fn frequencies_kf(v_str: &Vec<String>, f: &dyn Any) {
    match f.downcast_ref::<ArgFunHash>() {
        Some(fun) => {
            let freq = v_str.iter().fold(HashMap::new(), |mut hm, s| {
                *hm.entry(s.to_string()).or_insert(0) += 1;
                hm
            });

            fun(&freq); // call sort, next print
        }
        None => println!("Fail- frequencies\n"),
    }
}

pub fn sort_kf(words_f: &HashMap<String, i32>, f: &dyn Any) {
    match f.downcast_ref::<ArgFunVecPairs>() {
        Some(fun) => {
            let mut sort_v: Vec<(String, i32)> = words_f
                .into_iter()
                .map(|(a, b)| (String::from(a), *b))
                .collect();

            sort_v.sort_by_key(|k| -k.1);

            fun(&sort_v);
        }
        None => println!("Fail sort_kf"),
    }
}

pub fn print_kf(sort_v: &Vec<(String, i32)>, _f: &dyn Any) {
    println!(
        "{}",
        sort_v.iter().fold(String::new(), |mut s, e| {
            if e.1 > 4 {
                s.push_str(&format!("{} - {}\n", e.0, e.1));
            };
            s
        })
    );
}

pub fn do_nothing_kf() {}

pub fn kick_forward_test() {
    let file_name = String::from("data/text.txt");

    let do_nothing: Box<dyn Fn() -> ()> = Box::new(move || do_nothing_kf());

    let print_pairs: ArgFunVecPairs =
        Box::new(move |sort_v: &Vec<(String, i32)>| print_kf(sort_v, &do_nothing));

    let sort_hash: ArgFunHash =
        Box::new(move |words_f: &HashMap<String, i32>| sort_kf(words_f, &print_pairs));

    let frequencies_words: ArgFunVec =
        Box::new(move |v_str: &Vec<String>| frequencies_kf(v_str, &sort_hash));

    let remove_stop_wds: ArgFunVec =
        Box::new(move |v_str: &Vec<String>| remove_stop_words_kf(v_str, &frequencies_words));

    let scan_string: ArgFun = Box::new(move |text: &String| scan_string_kf(text, &remove_stop_wds));

    let filter: ArgFun =
        Box::new(move |text: &String| filter_chars_and_normalize_kf(text, &scan_string));

    let start_kf: ArgFun = Box::new(move |file_name: &String| read_file_kf(file_name, &filter));

    start_kf(&file_name);
}
