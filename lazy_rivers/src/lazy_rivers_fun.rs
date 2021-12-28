use std::collections::{HashMap, HashSet, LinkedList};
use std::fs::{self};
use std::iter::Iterator;
use std::str::Chars;

struct Characters {
    text: String,
}

impl Characters {
    fn new() -> Self {
        Characters {
            text: String::new(),
        }
    }

    fn from_file(&mut self, file_name: &String) -> Chars {
        self.text = fs::read_to_string(file_name).expect("Error");
        self.text.chars()
    }
}

struct AllWords {
    words: LinkedList<String>,
}

impl AllWords {
    fn new() -> Self {
        AllWords {
            words: LinkedList::new(),
        }
    }

    fn all_words(&mut self, file_name: &String) -> impl Iterator<Item = String> + '_ {
        let mut mi = Characters::new();

        let mut word = String::new();
        for c in mi.from_file(file_name) {
            if c.is_alphabetic() {
                word.push(c);
            } else {
                if word.len() > 0 {
                    self.words.push_back(word.to_lowercase().clone());
                    word.clear();
                }
            }
        }

        self.words.iter().cloned()
    }
}

struct FilterSW {
    words: LinkedList<String>,
    stop_w: HashSet<String>,
}

impl FilterSW {
    fn new(file_stop_w: &String) -> Self {
        FilterSW {
            words: LinkedList::new(),
            stop_w: fs::read_to_string(file_stop_w)
                .expect("error SW")
                .split(",")
                .map(|el| String::from(el))
                .collect(),
        }
    }

    fn filter_sw(&mut self, file_name: &String) -> impl Iterator<Item = String> + '_ {
        let mut aw = AllWords::new();

        for st in aw.all_words(file_name) {
            if !self.stop_w.contains(&st) && st.len() > 1 {
                self.words.push_back(st);
            }
        }

        self.words.iter().cloned()
    }
}

struct Frequencies {
    freqs: Vec<(String, i32)>,
}

impl Frequencies {
    fn new() -> Self {
        Frequencies { freqs: Vec::new() }
    }

    fn frequencies(
        &mut self,
        file_name: &String,
        file_stop_w: &String,
    ) -> impl Iterator<Item = (String, i32)> + '_ {
        let mut fsw = FilterSW::new(file_stop_w);

        let mut freqs: HashMap<String, i32> = HashMap::new();

        for st in fsw.filter_sw(file_name) {
            *freqs.entry(st).or_insert(0) += 1;
        }

        self.freqs = freqs.into_iter().collect();
        self.freqs.sort_by_key(|el| -el.1);

        self.freqs.iter().cloned()
    }
}

pub fn lazy_rivers_test(file_name: &String, file_stop_w: &String) {
    let mut frq = Frequencies::new();

    for fq in frq.frequencies(file_name, file_stop_w) {
        if fq.1 > 4 {
            println!("{} - {}", fq.0, fq.1);
        }
    }
}
