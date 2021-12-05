use regex::Regex;
use std::collections::{HashMap, HashSet, LinkedList};
use std::fs;

//////////////////////////////////////////////////////////////////
pub trait TFExercise {
    fn info(&self) -> String {
        String::from("trait TFExercise - not implemented;")
    }
}

//////////////////////////////////////////////////////////////////
pub struct DataStorageManager {
    data: String,
}

impl DataStorageManager {
    pub fn new(file_name: &String) -> Self {
        return DataStorageManager {
            data: fs::read_to_string(file_name).expect("Something went wrong reading the file"),
        };
    }

    pub fn words(&self) -> Box<LinkedList<String>> {
        Box::new(
            Regex::new(r"[\W_]+")
                .unwrap()
                .replace_all(&self.data.to_lowercase(), " ")
                .to_string()
                .split_whitespace()
                .fold(LinkedList::new(), |mut ll, el| {
                    ll.push_back(el.to_string());
                    ll
                }),
        )
    }
}

impl TFExercise for DataStorageManager {
    fn info(&self) -> String {
        String::from("I'm DataStorageManager")
    }
}

//////////////////////////////////////////////////////////////////
pub struct StopWordManager {
    stop_words: HashSet<String>,
}

impl StopWordManager {
    pub fn new(file_name: &String) -> Self {
        StopWordManager {
            stop_words: fs::read_to_string(file_name)
                .expect("something went wrong in reading stop words")
                .split(",")
                .fold(HashSet::new(), |mut hs, el| {
                    hs.insert(String::from(el));
                    hs
                }),
        }
    }

    pub fn is_stop_word(&self, w: &String) -> bool {
        self.stop_words.contains(w)
    }
}

impl TFExercise for StopWordManager {
    fn info(&self) -> String {
        String::from("I'm StopWordManager")
    }
}

//////////////////////////////////////////////////////////////////
pub struct WordFrequencyManager {
    word_freqs: HashMap<String, i32>,
}

impl WordFrequencyManager {
    pub fn new() -> Self {
        return WordFrequencyManager {
            word_freqs: HashMap::new(),
        };
    }

    pub fn increment_count(&mut self, word: &String) {
        if word.len() > 1 {
            *self.word_freqs.entry(String::from(word)).or_insert(0) += 1;
        }
    }

    pub fn sorted(&mut self) -> Box<Vec<(String, i32)>> {
        let mut vec_w: Box<Vec<(String, i32)>> = Box::new(Vec::new());

        for (ss, freq) in &self.word_freqs {
            vec_w.push((ss.clone(), freq.clone()));
        }
        vec_w.sort_by_key(|k| -k.1);
        vec_w
    }
}

impl TFExercise for WordFrequencyManager {
    fn info(&self) -> String {
        String::from("I'm WordFrequencyManager")
    }
}

//////////////////////////////////////////////////////////////////
struct WordFrequencyController {
    storage_manager: DataStorageManager,
    stop_word_manager: StopWordManager,
    word_freq_manager: WordFrequencyManager,
}

impl WordFrequencyController {
    pub fn new(file_name: &String, stop_w_file_name: &String) -> Self {
        return WordFrequencyController {
            storage_manager: DataStorageManager::new(file_name),
            stop_word_manager: StopWordManager::new(stop_w_file_name),
            word_freq_manager: WordFrequencyManager::new(),
        };
    }

    pub fn run(&mut self) -> String {
        let bw = self.storage_manager.words();
        for w in bw.iter() {
            if !self.stop_word_manager.is_stop_word(w) {
                self.word_freq_manager.increment_count(w);
            }
        }

        let word_freq = self.word_freq_manager.sorted();

        let mut res_str = String::new();

        for (word, freq) in word_freq.iter() {
            if *freq > 4 {
                res_str.push_str(&format!("{} - {}\n", word, freq));
            }
        }

        res_str
    }
}

pub fn things_test() {
    let file_name = String::from("data/text.txt");
    let stop_w_file = String::from("data/stop_words.txt");

    let mut wfc = WordFrequencyController::new(&file_name, &stop_w_file);
    println!("{}", wfc.run());
}
