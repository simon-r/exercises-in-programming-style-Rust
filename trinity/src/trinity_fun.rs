use regex::Regex;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::rc::Rc;

macro_rules! make_rc_refcell {
    ($value:expr) => {
        Rc::new(RefCell::new($value))
    };
}

struct WordFrequenciesModel {
    freq: HashMap<String, i32>,
}
type WFMRcCell = Rc<RefCell<WordFrequenciesModel>>;

impl WordFrequenciesModel {
    pub fn new() -> WordFrequenciesModel {
        WordFrequenciesModel {
            freq: HashMap::new(),
        }
    }

    pub fn update(&mut self, file_name: &String, file_stop_w: &String) {
        let stop_words = fs::read_to_string(file_stop_w)
            .expect("something went wrong in reading stop words")
            .split(",")
            .into_iter()
            .fold(HashSet::new(), |mut hs, w| {
                hs.insert(w.to_string());
                hs
            });

        self.freq = Regex::new(r"[\W_]+")
            .unwrap()
            .replace(
                &fs::read_to_string(file_name)
                    .expect("something went wrong")
                    .to_lowercase(),
                " ",
            )
            .to_lowercase()
            .split(" ")
            .into_iter()
            .fold(HashMap::<String, i32>::new(), |mut hm, w| {
                if !stop_words.contains(w) {
                    *hm.entry(w.to_string()).or_insert(0) += 1;
                }
                hm
            });
    }
}

struct WordFrequenciesView {
    model: WFMRcCell,
}
type WFVRcCell = Rc<RefCell<WordFrequenciesModel>>;

struct WordFrequencyController {
    model: WFMRcCell,
    view: WFVRcCell,
}

pub fn trinity_test(file_name: &String, file_stop_w: &String) {
    println!("trinity_test");
}
