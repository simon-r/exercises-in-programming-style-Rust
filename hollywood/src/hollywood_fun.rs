use regex::Regex;

use std::cell::RefCell;
use std::collections::{BTreeMap, LinkedList};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::rc::{Rc, Weak};

trait ProduceWordsEvent {
    fn produce_words(&mut self);
}

trait LoadDataEvent {
    fn load_data(&mut self, path_to_file: &String);
}

type RcCellLoadDataEvent = Rc<RefCell<dyn LoadDataEvent>>;

trait WordEvent {
    fn word(&mut self, w: &String);
}

type RcCellWordEv = Rc<RefCell<dyn WordEvent>>;

///////////////////////////////////////////////
trait WordFilterEvent {
    fn filter(&self, w: &String) -> bool;
}

struct StopWordsFilter {
    stop_words: HashSet<String>,
}

type RcCellSWF = Rc<RefCell<StopWordsFilter>>;
type RcCellSWFEv = Rc<RefCell<dyn WordFilterEvent>>;

impl StopWordsFilter {
    fn new() -> Self {
        StopWordsFilter {
            stop_words: HashSet::new(),
        }
    }

    fn new_rc_cell() -> RcCellSWF {
        Rc::new(RefCell::new(StopWordsFilter::new()))
    }
}

impl WordFilterEvent for StopWordsFilter {
    fn filter(&self, w: &String) -> bool {
        self.stop_words.contains(w)
    }
}

impl LoadDataEvent for StopWordsFilter {
    fn load_data(&mut self, path_to_file: &String) {
        self.stop_words = fs::read_to_string(path_to_file)
            .expect("something went wrong in reading stop words")
            .split(",")
            .into_iter()
            .fold(HashSet::new(), |mut hs, w| {
                hs.insert(w.to_string());
                hs
            });
    }
}

///////////////////////////////////////////////
struct DataStorage {
    data: String,
    stop_word_filter: RcCellSWFEv,
    word_events: LinkedList<RcCellWordEv>,
}

type RcCellDataStorage = Rc<RefCell<DataStorage>>;

impl DataStorage {
    fn new(swf_ev: &RcCellSWFEv) -> Self {
        DataStorage {
            data: String::new(),
            stop_word_filter: Rc::clone(swf_ev),
            word_events: LinkedList::new(),
        }
    }

    fn new_rc_cell(swf_ev: &RcCellSWFEv) -> RcCellDataStorage {
        Rc::new(RefCell::new(DataStorage::new(swf_ev)))
    }

    fn register_word_event(&mut self, w_ev: &RcCellWordEv) {
        self.word_events.push_back(Rc::clone(w_ev));
    }
}

impl LoadDataEvent for DataStorage {
    fn load_data(&mut self, file_name: &String) {
        self.data = Regex::new(r"[\W_]+")
            .unwrap()
            .replace_all(
                &fs::read_to_string(file_name)
                    .expect("something went wrong in reading stop words")
                    .to_lowercase(),
                " ",
            )
            .to_string();
    }
}

impl ProduceWordsEvent for DataStorage {
    fn produce_words(&mut self) {
        for word in self.data.split_whitespace() {
            if Rc::clone(&self.stop_word_filter)
                .borrow_mut()
                .filter(&word.to_string())
            {
                for we in &self.word_events {
                    Rc::clone(we).borrow_mut().word(&word.to_string());
                }
            }
        }
    }
}

//////////////////////////////////////////////////////////////
///
struct WordFrequencyCounter {
    words_freq: HashMap<String, i32>,
}

type RcCellWordFrequencyCounter = Rc<RefCell<WordFrequencyCounter>>;

impl WordFrequencyCounter {
    fn new() -> Self {
        WordFrequencyCounter {
            words_freq: HashMap::new(),
        }
    }

    fn new_rc_cell() -> RcCellWordFrequencyCounter {
        Rc::new(RefCell::new(WordFrequencyCounter::new()))
    }
}

impl WordEvent for WordFrequencyCounter {
    fn word(&mut self, w: &String) {
        *self.words_freq.entry(w.to_string()).or_insert(0) += 1;
    }
}

//////////////////////////////////////////////////////////////

struct WordFrequencyFramework {
    load_event_handlers: LinkedList<RcCellLoadDataEvent>,
}

impl WordFrequencyFramework {
    fn new() -> Self {
        WordFrequencyFramework {
            load_event_handlers: LinkedList::new(),
        }
    }

    fn register_load_data_ev(&mut self, ev: &RcCellLoadDataEvent) {
        self.load_event_handlers.push_back(Rc::clone(ev));
    }
}

//////////////////////////////////////////////////////////////
pub fn hollywood_test(file_name: &String, file_stop_w: &String) {
    let mut wf_framework = WordFrequencyFramework::new();

    let swf = StopWordsFilter::new_rc_cell();
    {
        let swf_ld = Rc::clone(&swf) as RcCellLoadDataEvent;
        wf_framework.register_load_data_ev(&swf_ld);
    }

    let swf_ev = Rc::clone(&swf) as RcCellSWFEv;
    let ds = DataStorage::new_rc_cell(&swf_ev);

    {
        let ds_ld = Rc::clone(&ds) as RcCellLoadDataEvent;
        wf_framework.register_load_data_ev(&ds_ld);
    }

    let wfc = WordFrequencyCounter::new_rc_cell();
    let wfc_wev = Rc::clone(&wfc) as RcCellWordEv;

    Rc::clone(&ds).borrow_mut().register_word_event(&wfc_wev);

    // Rc::clone(&swf).borrow_mut().load_data(file_stop_w);

    // let f = Rc::clone(&swf).borrow_mut().filter(&"the".to_string());
    // let swf_ev = Rc::clone(&swf) as RcCellSWFEv;

    // println!("{}", f);

    // let f2 = Rc::clone(&swf_ev).borrow_mut().filter(&"in".to_string());
    // println!("{}", f2);
}
