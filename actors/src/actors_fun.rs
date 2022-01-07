use regex::Regex;
use std::any::Any;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::rc::Rc;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

macro_rules! make_rc_refcell {
    ($value:expr) => {
        Rc::new(RefCell::new($value))
    };
}

macro_rules! make_arc_mutex {
    ($value:expr) => {
        Arc::new(Mutex::new($value))
    };
}

type RcRefAny = Rc<RefCell<dyn Any>>;
struct MessageAct {
    message: String,
    msg_data: RcRefAny,
}

pub struct DataStorageManager {
    data: String,
    recv: Arc<Mutex<Receiver<MessageAct>>>,
    send: Sender<MessageAct>,
}

impl DataStorageManager {
    pub fn new() -> Self {
        let (txm, rxm) = mpsc::channel::<MessageAct>();
        let rx_arcm = make_arc_mutex!(rxm);

        DataStorageManager {
            data: String::new(),
            recv: rx_arcm.clone(),
            send: txm.clone(),
        }
    }
}

fn recv(rr: Arc<Mutex<Receiver<i32>>>) -> JoinHandle<()> {
    let j = thread::spawn(move || {
        for xx in rr.clone().borrow_mut().lock().ok().unwrap().iter() {
            println!("{}", xx);
        }
    });

    j
}

pub fn actors_test(file_name: &String, file_stop_w: &String) {


    let mut dsm = DataStorageManager::new();

    let (tx, rx) = mpsc::channel::<i32>();
    let (txm, rxm) = mpsc::channel::<MessageAct>();

    let arc_rx = Arc::new(Mutex::new(rx));

    let arc_rxm = make_arc_mutex!(rxm);

    let j = recv(arc_rx);

    for i in 1..100 {
        tx.send(i);
    }

    j.join();
}
