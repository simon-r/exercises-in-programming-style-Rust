use regex::Regex;
use std::collections::{BTreeMap, HashMap, HashSet, LinkedList};
use std::fs;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

macro_rules! make_arc_mutex {
     ($value:expr) => {
         Arc::new(Mutex::new($value))
     };
 }

pub fn dataspaces_test(file_name: &String, file_stop_w: &String) {
    let mut word_space = make_arc_mutex!(LinkedList::<String>::new());
    let mut freq_space =  make_arc_mutex!(HashMap::<String, i32>::new());
}
