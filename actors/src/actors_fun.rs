use regex::Regex;
use std::any::Any;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::Hash;
use std::rc::Rc;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

macro_rules! make_arc_mutex {
    ($value:expr) => {
        Arc::new(Mutex::new($value))
    };
}

pub struct MessageStringAct {
    message: String,
    msg_data: String,
}

impl MessageStringAct {
    fn new(message: &String, msg_data: &String) -> Self {
        MessageStringAct {
            message: message.clone(),
            msg_data: msg_data.clone(),
        }
    }
}

///////////////////////////////////////////////
pub struct DataStorageManager {
    data: String,
    recv: Arc<Mutex<Receiver<MessageStringAct>>>,
    pub send: Sender<MessageStringAct>,
    // send_to_next: Sender<MessageStringAct>,
}

type ArcMutexDataStorageManager = Arc<Mutex<DataStorageManager>>;

impl DataStorageManager {
    pub fn new() -> Self {
        let (txm, rxm) = mpsc::channel::<MessageStringAct>();
        let rx_arcm = make_arc_mutex!(rxm);

        DataStorageManager {
            data: String::new(),
            recv: rx_arcm.clone(),
            send: txm.clone(),
        }
    }

    pub fn new_data_storage_listener() -> (
        ArcMutexDataStorageManager,
        Sender<MessageStringAct>,
        JoinHandle<()>,
    ) {
        let dsm_l = make_arc_mutex!(DataStorageManager::new());
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

    fn dispatch(&mut self) {
        for msg in self.recv.clone().lock().ok().unwrap().iter() {
            if msg.message == "kill" {
                break;
            } else if msg.message == "init" {
                let file_name = msg.msg_data.clone();
                self.init(&file_name);
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
}

pub fn actors_test(file_name: &String, file_stop_w: &String) {
    let (_dsm_l, send_dsm, j_dsm) = DataStorageManager::new_data_storage_listener();

    let _res = send_dsm.send(MessageStringAct::new(
        &String::from("init"),
        &String::from(file_name),
    ));

    let _res = send_dsm.send(MessageStringAct::new(
        &String::from("kill"),
        &String::from(file_name),
    ));

    j_dsm.join();
}
