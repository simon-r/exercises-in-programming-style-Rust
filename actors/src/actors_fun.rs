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
    FreqData((String, i32)),
    Eof,
    Empty,
    Kill,
}

pub struct MessageAct {
    message: String,
    msg_data: MessageData,
}

macro_rules! make_act_channel {
    () => {{
        let (send_msg, recv_msg) = mpsc::channel::<MessageAct>();
        let arc_mutex_recv_msg = make_arc_mutex!(recv_msg);
        (send_msg, arc_mutex_recv_msg)
    }};
}

macro_rules! freq_message {
    ($message:expr, $message_data:expr) => {
        MessageAct {
            message: $message.clone(),
            msg_data: MessageData::FreqData($message_data),
        }
    };
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

// impl MessageAct {
//     fn new(message: &String, msg_data: MessageData) -> Self {
//         MessageAct {
//             message: message.clone(),
//             msg_data: msg_data,
//         }
//     }
// }

///////////////////////////////////////////////
pub struct DataStorageManager {
    data: String,
    recv: Arc<Mutex<Receiver<MessageAct>>>,
    send_to_filter: Sender<MessageAct>,
}

type ArcMutexDataStorageManager = Arc<Mutex<DataStorageManager>>;

impl DataStorageManager {
    pub fn new(
        receiver: &Arc<Mutex<Receiver<MessageAct>>>,
        send_to_filter: &Sender<MessageAct>,
    ) -> Self {
        DataStorageManager {
            data: String::new(),
            recv: receiver.clone(),
            send_to_filter: send_to_filter.clone(),
        }
    }

    pub fn new_data_storage_listener(
        receiver: &Arc<Mutex<Receiver<MessageAct>>>,
        send_to_filter: &Sender<MessageAct>,
    ) -> (ArcMutexDataStorageManager, JoinHandle<()>) {
        let dsm_l = make_arc_mutex!(DataStorageManager::new(receiver, send_to_filter));

        let dsm_l_clone = dsm_l.clone();

        let join = thread::spawn(move || {
            dsm_l_clone.lock().ok().unwrap().dispatch();
        });

        (dsm_l, join)
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

        // println!("{}", data);
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
    recv: Arc<Mutex<Receiver<MessageAct>>>,
    send_to_next: Sender<MessageAct>,
}

type ArcMutexStopWordsManager = Arc<Mutex<StopWordsManager>>;

impl StopWordsManager {
    fn new(recv: &Arc<Mutex<Receiver<MessageAct>>>, send_to_next: &Sender<MessageAct>) -> Self {
        StopWordsManager {
            stop_words: HashSet::new(),
            recv: recv.clone(),
            send_to_next: send_to_next.clone(),
        }
    }

    fn new_stop_words_listener(
        recv: &Arc<Mutex<Receiver<MessageAct>>>,
        send_to_next: &Sender<MessageAct>,
    ) -> (ArcMutexStopWordsManager, JoinHandle<()>) {
        let sw_l = make_arc_mutex!(StopWordsManager::new(recv, send_to_next));

        let sw_l_clone = sw_l.clone();

        let join = thread::spawn(move || {
            sw_l_clone.lock().ok().unwrap().dispatch();
        });

        (sw_l, join)
    }

    fn dispatch(&mut self) {
        for msg in self.recv.clone().lock().ok().unwrap().iter() {
            if msg.message == "kill" {
                break;
            } else if msg.message == "init" {
                let file_name = match msg.msg_data {
                    MessageData::TextData(s) => String::from(s),
                    _ => {
                        assert!(false, "some error in SW init message");
                        String::new()
                    }
                };
                self.init(&file_name);
            } else if msg.message == "filter" {
                let word = match msg.msg_data {
                    MessageData::WordData(s) => String::from(s),
                    MessageData::Eof => String::new(),
                    _ => {
                        assert!(false, "some error in SW filter message");
                        String::new()
                    }
                };
                if word.len() > 0 {
                    self.filter(&word);
                } else {
                    let _ = self.send_to_next.send(eof_message!(String::from("word")));
                }
            }
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
        if !self.stop_words.contains(word) && word.len() > 1 {
            // println!("filter: {}", word);
            let _ = self
                .send_to_next
                .send(word_message!(String::from("word"), word));
        }
    }
}

//////////////////////////////////////////////////
struct FrequenciesManager {
    frequencies: HashMap<String, i32>,
    recv: Arc<Mutex<Receiver<MessageAct>>>,
    send_to_next: Sender<MessageAct>,
}

type ArcMutexFrequenciesManager = Arc<Mutex<FrequenciesManager>>;

impl FrequenciesManager {
    fn new(recv: &Arc<Mutex<Receiver<MessageAct>>>, send_to_next: &Sender<MessageAct>) -> Self {
        FrequenciesManager {
            frequencies: HashMap::new(),
            recv: recv.clone(),
            send_to_next: send_to_next.clone(),
        }
    }

    fn new_frequencies_listener(
        recv: &Arc<Mutex<Receiver<MessageAct>>>,
        send_to_next: &Sender<MessageAct>,
    ) -> (ArcMutexFrequenciesManager, JoinHandle<()>) {
        let fr_l = make_arc_mutex!(FrequenciesManager::new(recv, send_to_next));

        let fr_l_clone = fr_l.clone();

        let join = thread::spawn(move || {
            fr_l_clone.lock().ok().unwrap().dispatch();
        });

        (fr_l, join)
    }

    fn dispatch(&mut self) {
        for msg in self.recv.clone().lock().ok().unwrap().iter() {
            if msg.message == "kill" {
                break;
            } else if msg.message == "word" {
                match msg.msg_data {
                    MessageData::WordData(word) => {
                        self.update_freq(&word);
                    }
                    MessageData::Eof => {
                        self.send_frequencies();
                    }
                    _ => {
                        assert!(false, "some error in Freq message");
                    }
                };
            }
        }
    }

    fn update_freq(&mut self, word: &String) {
        *self.frequencies.entry(word.clone()).or_insert(0) += 1;
    }

    fn send_frequencies(&self) {
        for pair in self.frequencies.iter() {
            println!("send freq {} - {}", pair.0, pair.1);
            let _ = self.send_to_next.send(freq_message!(
                String::from("freq"),
                (pair.0.clone(), pair.1.clone())
            ));
        }
        let _ = self.send_to_next.send(eof_message!(String::from("freq")));
    }
}

pub fn actors_test(file_name: &String, file_stop_w: &String) {
    let (send_dsm, recv_dsm) = make_act_channel!();
    let (send_sw_filter, recv_sw_filter) = make_act_channel!();
    let (send_freq, recv_freq) = make_act_channel!();
    let (send_sort, recv_sort) = make_act_channel!();

    let (_dsm_l, j_dsm) = DataStorageManager::new_data_storage_listener(&recv_dsm, &send_sw_filter);
    let (_sw_l, j_sw) = StopWordsManager::new_stop_words_listener(&recv_sw_filter, &send_freq);
    let (_fr_l, j_fr) = FrequenciesManager::new_frequencies_listener(&recv_freq, &send_sort);

    let _res = send_dsm.send(text_message!(String::from("init"), file_name));
    let _res = send_sw_filter.send(text_message!(String::from("init"), file_stop_w));

    let _res = send_dsm.send(empty_message!(String::from("send_words")));
    // let _res = send_dsm.send(kill_message!());
    // let _res = send_sw_filter.send(kill_message!());

    j_dsm.join();
    j_sw.join();
    j_fr.join();
}
