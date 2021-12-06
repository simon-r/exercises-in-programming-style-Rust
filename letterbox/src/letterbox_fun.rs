use regex::Regex;
use std::any::Any;
use std::collections::BTreeMap;
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
            let bv = Box::new(self.data.split_whitespace().fold(Vec::new(), |mut vs, el| {
                vs.push(el.to_string());
                vs
            }));
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

//////////////////////////////////////////////////////////////////
pub struct WordFrequencyManager {
    word_freqs: HashMap<String, i32>,
}

//////////////////////////////////////////////////////////////////
struct WordFrequencyController {
    storage_manager: DataStorageManager,
    stop_word_manager: StopWordManager,
    word_freq_manager: WordFrequencyManager,
}

pub fn letterbox_test(file_name: &String, file_stop_w: &String) {
    let mut dsm: DataStorageManager = DataStorageManager::new();

    let  msg = MessageLB::new(&String::from("init"), Box::new(file_name.clone()));

    let  rr = dsm.dispatch(&msg);

    let text = rr.downcast_ref::<String>().unwrap().clone();

    let  msg_2 = MessageLB::new(&String::from("words"), Box::new(""));

    println!("{}", text);

    let  rr_2 = dsm.dispatch(&msg_2);

    let vs = rr_2.downcast_ref::<Vec<String>>().unwrap().clone();

    for el in vs.iter() {
        println!("{}", el);
    }
}
