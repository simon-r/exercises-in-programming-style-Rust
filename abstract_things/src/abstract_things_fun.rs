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
///
pub trait DataStorageManagerTrait {
    fn words(&self) -> Box<LinkedList<String>>;
}
pub struct DataStorageManager {
    data: String,
}

impl DataStorageManager {
    pub fn new(file_name: &String) -> Self {
        DataStorageManager {
            data: fs::read_to_string(file_name).expect("Something went wrong reading the file"),
        }
    }
}

impl DataStorageManagerTrait for DataStorageManager {
    fn words(&self) -> Box<LinkedList<String>> {
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
///
trait StopWordManagerTrait {
    fn is_stop_word(&self, w: &String) -> bool;
}
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
}

impl StopWordManagerTrait for StopWordManager {
    fn is_stop_word(&self, w: &String) -> bool {
        self.stop_words.contains(w)
    }
}

impl TFExercise for StopWordManager {
    fn info(&self) -> String {
        String::from("I'm StopWordManager")
    }
}

//////////////////////////////////////////////////////////////////
///

trait WordFrequencyManagerTrait {
    fn increment_count(&mut self, word: &String);
    fn sorted(&mut self) -> Box<Vec<(String, i32)>>;
}

pub struct WordFrequencyManager {
    word_freqs: HashMap<String, i32>,
}

impl WordFrequencyManager {
    pub fn new() -> Self {
        return WordFrequencyManager {
            word_freqs: HashMap::new(),
        };
    }
}

impl WordFrequencyManagerTrait for WordFrequencyManager {
    fn increment_count(&mut self, word: &String) {
        if word.len() > 1 {
            *self.word_freqs.entry(String::from(word)).or_insert(0) += 1;
        }
    }

    fn sorted(&mut self) -> Box<Vec<(String, i32)>> {
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
    storage_manager: Box<dyn DataStorageManagerTrait>,
    stop_word_manager: Box<dyn StopWordManagerTrait>,
    word_freq_manager: Box<dyn WordFrequencyManagerTrait>,
}

impl WordFrequencyController {
    pub fn new(
        box_sm: Box<dyn DataStorageManagerTrait>,
        box_swm: Box<dyn StopWordManagerTrait>,
        box_wfm: Box<dyn WordFrequencyManagerTrait>,
    ) -> Self {
        WordFrequencyController {
            storage_manager: box_sm,
            stop_word_manager: box_swm,
            word_freq_manager: box_wfm,
        }
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

pub fn abstract_things_test(file_name: &String, file_stop_w: &String) {
    let box_sm: Box<dyn DataStorageManagerTrait> = Box::new(DataStorageManager::new(file_name));
    let box_swm: Box<dyn StopWordManagerTrait> = Box::new(StopWordManager::new(&file_stop_w));
    let box_wfm: Box<dyn WordFrequencyManagerTrait> = Box::new(WordFrequencyManager::new());

    let mut wfc = WordFrequencyController::new(box_sm, box_swm, box_wfm);

    let res_str = wfc.run();

    println!("{}", res_str);
}
