use regex::Regex;
use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::fs;

//////////////////////////////////////////////////////////////////
struct MessageLB {
    message: String,
    msg_data: Box<dyn Any>,
}

impl MessageLB {
    pub fn new(msg: &String, data: Box<dyn Any>) -> Self {
        MessageLB {
            message: String::from(msg),
            msg_data: data,
        }
    }
}

//////////////////////////////////////////////////////////////////
trait Dispatcher {
    fn dispatch(&mut self, message: &MessageLB) -> Box<dyn Any>;
}

//////////////////////////////////////////////////////////////////
pub struct DataStorageManager {
    data: String,
}

impl Dispatcher for DataStorageManager {
    fn dispatch(&mut self, message: &MessageLB) -> Box<dyn Any> {
        if message.message == String::from("init") {
            let error_file_name = String::from("_____error_fn____dispatch");

            let file_name = message
                .msg_data
                .downcast_ref::<String>()
                .unwrap_or(&error_file_name);

            if *file_name != error_file_name {
                return Box::new(self.init(file_name));
            } else {
                assert!(false);
                return Box::new("");
            }
        } else if message.message == String::from("words") {
            let bv = Box::new(self.words());
            return bv;
        }

        assert!(false, "Message not understood!!!\n");
        return Box::new("");
    }
}

impl DataStorageManager {
    pub fn new() -> Self {
        DataStorageManager {
            data: String::from(""),
        }
    }

    fn init(&mut self, file_name: &String) -> String {
        let text = fs::read_to_string(file_name)
            .expect("Error in reading text")
            .to_string();

        self.data = Regex::new(r"[\W_]+")
            .unwrap()
            .replace_all(&text.to_lowercase(), " ")
            .to_string();

        self.data.clone()
    }

    fn words(&mut self) -> Vec<String> {
        self.data.split_whitespace().fold(Vec::new(), |mut vs, el| {
            vs.push(el.to_string());
            vs
        })
    }
}

//////////////////////////////////////////////////////////////////
pub struct StopWordManager {
    stop_words: HashSet<String>,
}

impl StopWordManager {
    fn new() -> Self {
        StopWordManager {
            stop_words: HashSet::new(),
        }
    }

    fn init(&mut self, file_stop_w: &String) -> Box<dyn Any> {
        self.stop_words = fs::read_to_string(file_stop_w)
            .expect("Fail in read stop words")
            .to_string()
            .split(",")
            .fold(HashSet::new(), |mut hs, el| {
                hs.insert(el.to_string());
                hs
            });

        Box::new(true)
    }

    fn is_stop_word(&mut self, word: &String) -> Box<dyn Any> {
        Box::new(self.stop_words.contains(word))
    }
}

impl Dispatcher for StopWordManager {
    fn dispatch(&mut self, message: &MessageLB) -> Box<dyn Any> {
        let msg_data = match message.msg_data.downcast_ref::<String>() {
            Some(str) => String::from(str),
            None => {
                assert!(false);
                String::from("")
            }
        };

        if message.message == "init" {
            self.init(&msg_data)
        } else if message.message == "is_stop_word" {
            self.is_stop_word(&msg_data)
        } else {
            assert!(false);
            Box::new(false)
        }
    }
}

//////////////////////////////////////////////////////////////////
pub struct WordFrequencyManager {
    word_freqs: HashMap<String, i32>,
}

impl Dispatcher for WordFrequencyManager {
    fn dispatch(&mut self, message: &MessageLB) -> Box<dyn Any> {
        if message.message == "increment_count" {
            let msg_data = match message.msg_data.downcast_ref::<String>() {
                Some(str) => String::from(str),
                None => {
                    assert!(false);
                    String::from("")
                }
            };

            self.increment_count(&msg_data);
            Box::new(true)
        } else if message.message == "sorted" {
            self.sorted()
        } else {
            assert!(false);
            Box::new(false)
        }
    }
}

impl WordFrequencyManager {
    fn new() -> Self {
        WordFrequencyManager {
            word_freqs: HashMap::new(),
        }
    }

    fn increment_count(&mut self, word: &String) {
        *self.word_freqs.entry(word.clone()).or_insert(0) += 1;
    }

    fn sorted(&mut self) -> Box<dyn Any> {
        let mut ves: Vec<(String, i32)> = Vec::new();
        ves.reserve(self.word_freqs.len());

        for (st, freq) in &self.word_freqs {
            if *freq > 4 {
                ves.push((st.clone(), freq.clone()));
            }
        }

        ves.sort_by_key(|el| -el.1);
        Box::new(ves)
    }
}

//////////////////////////////////////////////////////////////////
struct WordFrequencyController {
    storage_manager: DataStorageManager,
    stop_word_manager: StopWordManager,
    word_freq_manager: WordFrequencyManager,
}

impl Dispatcher for WordFrequencyController {
    fn dispatch(&mut self, message: &MessageLB) -> Box<dyn Any> {
        if message.message == "init" {
            let msg_data = match message.msg_data.downcast_ref::<Vec<String>>() {
                Some(vs) => vs.clone(),
                None => {
                    assert!(false);
                    Vec::new() as Vec<String>
                }
            };

            self.init(&msg_data[0], &msg_data[1]);
            Box::new(true)
        } else if message.message == "run" {
            self.run()
        } else {
            assert!(false);
            Box::new(false)
        }
    }
}

impl WordFrequencyController {
    fn new() -> Self {
        WordFrequencyController {
            stop_word_manager: StopWordManager::new(),
            storage_manager: DataStorageManager::new(),
            word_freq_manager: WordFrequencyManager::new(),
        }
    }

    fn init(&mut self, file_name: &String, file_stop_w: &String) {
        let msg_sm = MessageLB::new(&"init".to_string(), Box::new(file_name.to_string()));
        self.storage_manager.dispatch(&msg_sm);

        let msg_sw = MessageLB::new(&"init".to_string(), Box::new(file_stop_w.to_string()));
        self.stop_word_manager.dispatch(&msg_sw);
    }

    fn run(&mut self) -> Box<dyn Any> {
        let msg_words = MessageLB::new(&"words".to_string(), Box::new(false));
        let box_vs = self.storage_manager.dispatch(&msg_words);

        for w in box_vs.downcast_ref::<Vec<String>>().unwrap().iter() {
            let msg_stop_w = MessageLB::new(&"is_stop_word".to_string(), Box::new(w.clone()));

            if !*self
                .stop_word_manager
                .dispatch(&msg_stop_w)
                .downcast_ref::<bool>()
                .unwrap()
            {
                let msg_freq_manager =
                    MessageLB::new(&"increment_count".to_string(), Box::new(w.clone()));
                self.word_freq_manager.dispatch(&msg_freq_manager);
            }
        }

        let msg_sorted = MessageLB::new(&"sorted".to_string(), Box::new(true));
        let res_str = self
            .word_freq_manager
            .dispatch(&msg_sorted)
            .downcast_ref::<Vec<(String, i32)>>()
            .unwrap()
            .iter()
            .fold(String::from(""), |mut ss, el| {
                ss.push_str(&format!("{} - {}\n", el.0, el.1));
                ss
            });

        Box::new(res_str)
    }
}

pub fn letterbox_test(file_name: &String, file_stop_w: &String) {
    let mut wfc = WordFrequencyController::new();

    let msg_init = MessageLB::new(
        &"init".to_string(),
        Box::new(vec![file_name.clone(), file_stop_w.clone()]),
    );
    let msg_run = MessageLB::new(&"run".to_string(), Box::new(true));

    wfc.dispatch(&msg_init);
    let res = wfc
        .dispatch(&msg_run)
        .downcast_ref::<String>()
        .unwrap()
        .clone();

    println!("{}", res);
}
