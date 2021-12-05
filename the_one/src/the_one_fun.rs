use regex::Regex;
use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::fs;

pub type VecStrInt = Vec<(String, i32)>;
pub type BindFun = Box<dyn Fn(&Box<dyn Any>) -> Box<dyn Any>>;

struct TFTheOne {
    value: Box<dyn Any>,
}

impl TFTheOne {
    fn new(file_name: &String) -> Self {
        return TFTheOne {
            value: Box::new(file_name.clone()),
        };
    }

    fn print_me(&self) {
        match self.value.downcast_ref::<VecStrInt>() {
            Some(val) => {
                let str_res = val.iter().fold(String::new(), |mut ss, pair| {
                    if pair.1 > 4 {
                        ss.push_str(&format!("{} - {}\n", pair.0, pair.1))
                    };
                    ss
                });

                println!("{}", str_res);
            }
            None => {
                println!("Error - print_me");
            }
        }
    }

    fn bind(&mut self, fun: &BindFun) -> &mut TFTheOne {
        self.value = fun(&self.value);
        self
    }
}

pub fn read_file(val: &Box<dyn Any>) -> Box<dyn Any> {
    match val.downcast_ref::<String>() {
        Some(file_name) => {
            let text =
                fs::read_to_string(file_name).expect("Something went wrong reading the file");

            let bx: Box<dyn Any> = Box::new(text);
            return bx;
        }
        None => {
            assert!(true, "Error: read_file");
            let bx: Box<dyn Any> = Box::new(String::from("Error"));
            return bx;
        }
    }
}

pub fn filter_chars_and_normalize(val: &Box<dyn Any>) -> Box<dyn Any> {
    match val.downcast_ref::<String>() {
        Some(in_text) => {
            let text = Regex::new(r"[\W_]+")
                .unwrap()
                .replace_all(&in_text.to_lowercase(), " ")
                .to_string();

            let bx: Box<dyn Any> = Box::new(text);
            return bx;
        }
        None => {
            assert!(true, "Error: filter_chars");
            let bx: Box<dyn Any> = Box::new(String::from("Error"));
            return bx;
        }
    }
}

pub fn scan_str(val: &Box<dyn Any>) -> Box<dyn Any> {
    match val.downcast_ref::<String>() {
        Some(in_text) => {
            let vec_words = in_text.split_whitespace().filter(|val| val.len() > 1).fold(
                Vec::new(),
                |mut vstr, val| {
                    vstr.push(val.to_string());
                    vstr
                },
            );

            let bx: Box<dyn Any> = Box::new(vec_words);
            return bx;
        }
        None => {
            assert!(true, "Error: filter_chars");
            let bx: Box<dyn Any> = Box::new(String::from("Error"));
            return bx;
        }
    }
}

pub fn remove_stop_words(val: &Box<dyn Any>, stop_w_file: &String) -> Box<dyn Any> {
    let stop_w: HashSet<String> = fs::read_to_string(stop_w_file)
        .expect("Something went wrong reading the stopw.txt file")
        .split(",")
        .fold(HashSet::new(), |mut hs, el| {
            hs.insert(String::from(el));
            hs
        });

    match val.downcast_ref::<Vec<String>>() {
        Some(in_vect_text) => {
            let vec_words: Vec<String> = in_vect_text
                .iter()
                .filter(|val| !stop_w.contains(&val.to_string()))
                .fold(Vec::new(), |mut vstr, val| {
                    vstr.push(val.to_string());
                    vstr
                });

            let bx: Box<dyn Any> = Box::new(vec_words);
            return bx;
        }
        None => {
            assert!(true, "Error: remove_stop_words");
            let bx: Box<dyn Any> = Box::new(String::from("Error"));
            return bx;
        }
    }
}

pub fn frequencies_and_sort(val: &Box<dyn Any>) -> Box<dyn Any> {
    match val.downcast_ref::<Vec<String>>() {
        Some(in_vec_words) => {
            let mut vec_freq = in_vec_words
                .iter()
                .fold(HashMap::new(), |mut hm, s| {
                    *hm.entry(s.to_string()).or_insert(0) += 1;
                    hm
                })
                .into_iter()
                .fold(Vec::new(), |mut vstr, val| {
                    vstr.push(val);
                    vstr
                });

            vec_freq.sort_by_key(|el| -el.1);

            let bx: Box<dyn Any> = Box::new(vec_freq);
            return bx;
        }
        None => {
            assert!(true, "Error: frequencies_and_sort");
            let bx: Box<dyn Any> = Box::new(String::from("Error"));
            return bx;
        }
    }
}

pub fn the_one_test() {
    let file_name = String::from("data/text.txt");
    let stop_w_file = String::from("data/stop_words.txt");

    let mut the_one = TFTheOne::new(&file_name);

    let read_file_lam: BindFun = Box::new(|val: &Box<dyn Any>| read_file(val));

    let filter_chars_and_normalize_lam: BindFun =
        Box::new(|val: &Box<dyn Any>| filter_chars_and_normalize(val));

    let scan_str_lam: BindFun = Box::new(|val: &Box<dyn Any>| scan_str(val));

    let remove_stop_words_lam: BindFun =
        Box::new(move |val: &Box<dyn Any>| remove_stop_words(val, &stop_w_file));

    let frequencies_lam: BindFun = Box::new(|val: &Box<dyn Any>| frequencies_and_sort(val));

    the_one
        .bind(&read_file_lam)
        .bind(&filter_chars_and_normalize_lam)
        .bind(&scan_str_lam)
        .bind(&remove_stop_words_lam)
        .bind(&frequencies_lam)
        .print_me();
}
