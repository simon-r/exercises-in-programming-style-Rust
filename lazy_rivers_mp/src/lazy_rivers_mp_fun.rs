use std::borrow::BorrowMut;
use std::collections::BTreeMap;
use std::collections::{HashMap, HashSet};
use std::fs::{self};
use std::iter::Iterator;
use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

fn from_file(file_name: &String, tx: &Sender<Option<char>>) {
    let text = fs::read_to_string(file_name).expect("Error in reading text");

    {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            for c in text.chars() {
                tx_clone.send(Some(c.clone())).unwrap();
            }
            tx_clone.send(None).unwrap();
        });
    }
}

fn all_words(send_to_next: &Sender<Option<String>>) -> Sender<Option<char>> {
    let (tx, rx) = mpsc::channel::<Option<char>>();

    let send_to_next_clone = send_to_next.clone();

    thread::spawn(move || {
        let mut word = String::new();
        for val in rx {
            match val {
                Some(c) => {
                    if c.is_alphabetic() {
                        word.push(c);
                    } else {
                        if word.len() > 0 {
                            send_to_next_clone
                                .send(Some(word.to_lowercase().clone()))
                                .unwrap();
                            word.clear();
                        }
                    }
                }
                None => {
                    send_to_next_clone.send(None).unwrap();
                    break;
                }
            }
        }
    });

    tx
}

fn filter_stop_words(
    file_stop_w: &String,
    send_to_next: &Sender<Option<String>>,
) -> Sender<Option<String>> {
    let stop_w = fs::read_to_string(file_stop_w)
        .expect("error SW")
        .split(",")
        .map(|el| String::from(el))
        .collect::<HashSet<_>>();

    let (tx, rx) = mpsc::channel::<Option<String>>();
    let send_to_next_clone = send_to_next.clone();

    thread::spawn(move || {
        for os in rx {
            match os {
                Some(word) => {
                    if !stop_w.contains(&word) {
                        send_to_next_clone.send(Some(word.clone())).unwrap();
                    }
                }
                None => {
                    send_to_next_clone.send(None).unwrap();
                    break;
                }
            }
        }
    });

    tx
}

fn frequencies(send_to_next: &Sender<Option<(String, i32)>>) -> Sender<Option<String>> {
    let (tx, rx) = mpsc::channel::<Option<String>>();

    let send_to_next_clone = send_to_next.clone();

    thread::spawn(move || {
        let mut freqs: HashMap<String, i32> = HashMap::new();

        for op in rx {
            match op {
                Some(word) => {
                    *freqs.entry(word).or_insert(0) += 1;
                }
                None => {
                    break;
                }
            }
        }

        let _cnt = freqs
            .iter()
            .map(|pair| {
                send_to_next_clone
                    .send(Some((pair.0.clone(), pair.1.clone())))
                    .unwrap();
                true
            })
            .count();

        send_to_next_clone.send(None).unwrap();
    });

    tx
}

fn sort_words(
    top: i32,
    send_to_next: &Sender<Option<(String, i32)>>,
) -> Sender<Option<(String, i32)>> {
    let (tx, rx) = mpsc::channel::<Option<(String, i32)>>();

    let send_to_next_clone = send_to_next.clone();

    thread::spawn(move || {
        let mut bt = BTreeMap::<(i32, String), bool>::new();

        for op in rx {
            match op {
                Some(pair) => {
                    bt.insert((pair.1, pair.0), true);
                }
                None => {
                    break;
                }
            }
        }

        for pair in bt.into_iter().rev() {
            if pair.0 .0 > top {
                send_to_next_clone
                    .send(Some((pair.0 .1.clone(), pair.0 .0.clone())))
                    .unwrap();
            }
        }
        send_to_next_clone.send(None).unwrap();
    });

    tx
}

fn string_accumulator(sta: Arc<Mutex<String>>) -> (Sender<Option<(String, i32)>>, JoinHandle<()>) {
    let (tx, rx) = mpsc::channel::<Option<(String, i32)>>();
    let mut sta_clone = sta.clone();

    let j = thread::spawn(move || {
        for received in rx {
            match received {
                Some(c) => sta_clone
                    .borrow_mut()
                    .lock()
                    .ok()
                    .unwrap()
                    .push_str(&format!("{} - {}\n", c.0, c.1)),
                None => break,
            }
        }
    });

    (tx, j)
}

pub fn lazy_rivers_mp_test(file_name: &String, file_stop_w: &String) {
    // let (tx, rx) = mpsc::channel::<Option<(String, i32)>>();

    let mut string_acc = Arc::new(Mutex::new(String::new()));

    let (send_awords, j) = string_accumulator(string_acc.clone());
    let send_swords = sort_words(4, &send_awords);
    let send_fwords = frequencies(&send_swords);
    let send_words = filter_stop_words(file_stop_w, &send_fwords);
    let send_chars = all_words(&send_words);
    from_file(file_name, &send_chars);

    match j.join() {
        Ok(_v) => {
            println!("\n{}", string_acc.borrow_mut().lock().ok().unwrap());
        }
        Err(_e) => {
            println!("Some error")
        }
    }
}
