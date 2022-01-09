use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

macro_rules! make_arc_mutex {
    ($value:expr) => {
        Arc::new(Mutex::new($value))
    };
}

enum MessageData {
    TextData(String),
    WordData(String),
    Eof,
    Empty,
    Kill,
}

pub struct MessageAct {
    message: String,
    msg_data: MessageData,
}

macro_rules! text_message {
    ($message:expr, $message_data:expr) => {
        MessageAct {
            message: $message.clone(),
            msg_data: MessageData::TextData($message_data.clone()),
        }
    };
}

macro_rules! word_message {
    ($message:expr, $message_data:expr) => {
        MessageAct {
            message: $message.clone(),
            msg_data: MessageData::WordData($message_data.clone()),
        }
    };
}

macro_rules! empty_message {
    ($message:expr) => {
        MessageAct {
            message: $message.clone(),
            msg_data: MessageData::Empty,
        }
    };
}

macro_rules! eof_message {
    ($message:expr) => {
        MessageAct {
            message: $message.clone(),
            msg_data: MessageData::Eof,
        }
    };
}

macro_rules! kill_message {
    () => {
        MessageAct {
            message: String::from("kill"),
            msg_data: MessageData::Kill,
        }
    };
}

impl MessageAct {
    fn new(message: &String, msg_data: MessageData) -> Self {
        MessageAct {
            message: message.clone(),
            msg_data: msg_data,
        }
    }
}

///////////////////////////////////////////////
pub struct DataStorageManager {
    data: String,
    recv: Arc<Mutex<Receiver<MessageAct>>>,
    pub send: Sender<MessageAct>,
    send_to_filter: Sender<MessageAct>,
}

type ArcMutexDataStorageManager = Arc<Mutex<DataStorageManager>>;

impl DataStorageManager {
    pub fn new(send_to_filter: &Sender<MessageAct>) -> Self {
        let (txm, rxm) = mpsc::channel::<MessageAct>();
        let rx_arcm = make_arc_mutex!(rxm);

        DataStorageManager {
            data: String::new(),
            recv: rx_arcm.clone(),
            send: txm.clone(),
            send_to_filter: send_to_filter.clone(),
        }
    }

    pub fn new_data_storage_listener(
        send_to_filter: &Sender<MessageAct>,
    ) -> (
        ArcMutexDataStorageManager,
        Sender<MessageAct>,
        JoinHandle<()>,
    ) {
        let dsm_l = make_arc_mutex!(DataStorageManager::new(send_to_filter));
        let dsm_send = dsm_l.lock().unwrap().send.clone();

        let dsm_l_clone = dsm_l.clone();

        let join = thread::spawn(move || {
            dsm_l_clone.lock().ok().unwrap().dispatch();
        });

        (dsm_l, dsm_send, join)
    }

    fn init(&mut self, file_name: &String) {
        let data = Regex::new(r"[\W_]+")
            .unwrap()
            .replace_all(
                &fs::read_to_string(file_name)
                    .expect("some error in read data")
                    .to_lowercase(),
                " ",
            )
            .to_string();

        println!("{}", data);
        self.data = data;
    }

    fn send_words(&self) {
        for word in self.data.split(" ").map(|el| String::from(el)) {
            let _rs = self
                .send_to_filter
                .send(word_message!(String::from("filter"), word.clone()));
        }
        let _rs = self
            .send_to_filter
            .send(eof_message!(String::from("filter")));
    }

    fn dispatch(&mut self) {
        for msg in self.recv.clone().lock().ok().unwrap().iter() {
            if msg.message == "kill" {
                break;
            } else if msg.message == "init" {
                let file_name = match msg.msg_data {
                    MessageData::TextData(s) => String::from(s),
                    _ => {
                        assert!(false, "some error in init message");
                        String::new()
                    }
                };
                self.init(&file_name);
            } else if msg.message == "send_words" {
                self.send_words();
            }
        }
    }
}

//////////////////////////////////////////////////
struct StopWordsManager {
    stop_words: HashSet<String>,
}

impl StopWordsManager {
    fn new() -> Self {
        StopWordsManager {
            stop_words: HashSet::new(),
        }
    }

    fn init(&mut self, file_stop_w: &String) {
        self.stop_words = fs::read_to_string(file_stop_w)
            .expect("error SW")
            .split(",")
            .map(|el| String::from(el))
            .collect::<HashSet<_>>();
    }

    fn filter(&self, word: &String) {
        if !self.stop_words.contains(word) && word.len() > 1 {}
    }
}

pub fn actors_test(file_name: &String, file_stop_w: &String) {
    let (tx, rx) = mpsc::channel::<MessageAct>();

    let (_dsm_l, send_dsm, j_dsm) = DataStorageManager::new_data_storage_listener(&tx);

    let _res = send_dsm.send(text_message!(String::from("init"), file_name));
    let _res = send_dsm.send(empty_message!(String::from("send_words")));
    let _res = send_dsm.send(kill_message!());

    j_dsm.join();
}
