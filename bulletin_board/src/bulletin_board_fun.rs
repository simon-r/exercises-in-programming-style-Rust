use regex::Regex;
use std::any::Any;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, LinkedList};
use std::fs;
use std::rc::Rc;
use std::rc::Weak;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum MyEvent {
    OnLoad,
    OnLoadStopWords,
    OnStart,
    OnWord,
    OnEof,
    OnValidWord,
    OnPrint,
}

type EventArg = Rc<dyn Any>;
type EventHandlerType = Rc<RefCell<dyn Fn(EventArg)>>;

struct EventManager {
    events_map: HashMap<MyEvent, LinkedList<EventHandlerType>>,
    weak_self: Weak<RefCell<Self>>,
}

type RcCellEventManager = Rc<RefCell<EventManager>>;

impl EventManager {
    fn new() -> Self {
        EventManager {
            events_map: HashMap::new(),
            weak_self: Weak::new(),
        }
    }

    fn new_rc_cell() -> RcCellEventManager {
        let evm = Rc::new(RefCell::new(EventManager::new()));
        Rc::clone(&evm).borrow_mut().weak_self = Rc::downgrade(&evm);
        evm
    }

    fn register_event(&mut self, ev: &MyEvent, h_fun: &EventHandlerType) {
        self.events_map
            .entry(*ev)
            .or_insert(LinkedList::new())
            .push_back(Rc::clone(h_fun));
    }

    fn emit(&self, ev: &MyEvent, arg: &EventArg) {
        if !self.events_map.contains_key(ev) {
            return;
        }

        let ev_list = self.events_map.get(ev).unwrap();

        for h_fun in ev_list {
            Rc::clone(h_fun).borrow()(Rc::clone(arg));
        }
    }
}

/////////////////////////////////////////////////////////////////
struct DataStorage {
    text: String,
    weak_self: Weak<RefCell<Self>>,
    event_manager: Option<RcCellEventManager>,
}

impl DataStorage {
    fn new() -> Self {
        DataStorage {
            text: String::new(),
            weak_self: Weak::new(),
            event_manager: None,
        }
    }

    fn new_rc_cell() -> Rc<RefCell<DataStorage>> {
        let ds = Rc::new(RefCell::new(DataStorage::new()));
        Rc::clone(&ds).borrow_mut().weak_self = Rc::downgrade(&ds);
        ds
    }

    fn register_events(&mut self, rc_ev_manager: &RcCellEventManager) {
        {
            let self_rc = self.weak_self.upgrade().unwrap().clone();

            let load_data_h: EventHandlerType = Rc::new(RefCell::new(
                //
                move |arg: Rc<dyn Any>| {
                    let file_name = arg.downcast_ref::<String>();
                    Rc::clone(&self_rc)
                        .borrow_mut()
                        .load_data(file_name.unwrap());
                },
            ));

            Rc::clone(rc_ev_manager)
                .borrow_mut()
                .register_event(&MyEvent::OnLoad, &load_data_h);
        }

        {
            let self_rc = self.weak_self.upgrade().unwrap().clone();

            let produce_words_h: EventHandlerType = Rc::new(RefCell::new(
                //
                move |_arg: Rc<dyn Any>| {
                    Rc::clone(&self_rc).borrow().produce_words();
                },
            ));

            Rc::clone(rc_ev_manager)
                .borrow_mut()
                .register_event(&MyEvent::OnStart, &produce_words_h);
        }

        self.event_manager = Some(rc_ev_manager.clone());
    }

    fn load_data(&mut self, file_name: &String) {
        self.text = fs::read_to_string(file_name)
            .expect("Some error in reading data!")
            .to_lowercase();
        // println!("{}", self.text);
    }

    fn produce_words(&self) {
        let word_list = Regex::new(r"[\W_]+")
            .unwrap()
            .replace_all(&self.text.to_lowercase(), " ")
            .split_whitespace()
            .map(String::from)
            .collect::<LinkedList<String>>();

        for word in word_list {
            let evm = Rc::clone(self.event_manager.as_ref().unwrap());
            // println!("word: {}", &word);

            Rc::clone(&evm)
                .borrow()
                .emit(&MyEvent::OnWord, &(Rc::new(word.clone()) as Rc<dyn Any>));
        }
        {
            let evm = Rc::clone(self.event_manager.as_ref().unwrap());
            Rc::clone(&evm)
                .borrow()
                .emit(&MyEvent::OnEof, &(Rc::new(()) as Rc<dyn Any>));
        }
    }
}

/////////////////////////////////////////////////////////////////
struct StopWords {
    stop_words: HashSet<String>,
    weak_self: Weak<RefCell<Self>>,
    event_manager: Option<RcCellEventManager>,
}

type RcCellStopWords = Rc<RefCell<StopWords>>;

impl StopWords {
    fn new() -> Self {
        StopWords {
            stop_words: HashSet::new(),
            weak_self: Weak::new(),
            event_manager: None,
        }
    }

    fn new_rc_cell() -> RcCellStopWords {
        let stw = Rc::new(RefCell::new(StopWords::new()));
        Rc::clone(&stw).borrow_mut().weak_self = Rc::downgrade(&stw);
        stw
    }

    fn load_stop_words(&mut self, file_stop_w: &String) {
        self.stop_words = fs::read_to_string(file_stop_w)
            .expect("Some error in reading stop words")
            .split(",")
            .map(String::from)
            .collect::<HashSet<String>>();

        // for w in &self.stop_words {
        //     println!("{}", w);
        // }
    }

    fn is_stop_word(&self, word: &String) -> bool {
        self.stop_words.contains(word)
    }

    fn register_events(&mut self, rc_ev_manager: &RcCellEventManager) {
        {
            // Load Stop words
            let self_rc = self.weak_self.upgrade().unwrap().clone();

            let load_stop_words_h: EventHandlerType = Rc::new(RefCell::new(
                //
                move |arg: Rc<dyn Any>| {
                    let file_name = arg.downcast_ref::<String>();
                    Rc::clone(&self_rc)
                        .borrow_mut()
                        .load_stop_words(file_name.unwrap());
                },
            ));

            Rc::clone(rc_ev_manager)
                .borrow_mut()
                .register_event(&MyEvent::OnLoadStopWords, &load_stop_words_h);
        }

        {
            // Validate word
            let self_rc = self.weak_self.upgrade().unwrap().clone();
            let rc_ev_manager_cp = Rc::clone(rc_ev_manager);

            let validate_word_h: EventHandlerType = Rc::new(RefCell::new(
                //
                move |arg: Rc<dyn Any>| {
                    let word = arg.downcast_ref::<String>().unwrap();
                    let f = Rc::clone(&self_rc).borrow().is_stop_word(word);
                    if !f {
                        Rc::clone(&rc_ev_manager_cp).borrow().emit(
                            &MyEvent::OnValidWord,
                            &(Rc::new(word.clone()) as Rc<dyn Any>),
                        );
                    }
                },
            ));

            Rc::clone(rc_ev_manager)
                .borrow_mut()
                .register_event(&MyEvent::OnWord, &validate_word_h);
        }

        self.event_manager = Some(rc_ev_manager.clone());
    }
}

/////////////////////////////////////////////////////////////////////
///
struct WordsFrequencies {
    words_frequencies: HashMap<String, i32>,
    weak_self: Weak<RefCell<Self>>,
    event_manager: Option<RcCellEventManager>,
}

type RcCellWordsFrequencies = Rc<RefCell<WordsFrequencies>>;

impl WordsFrequencies {
    fn new() -> Self {
        WordsFrequencies {
            words_frequencies: HashMap::new(),
            weak_self: Weak::new(),
            event_manager: None,
        }
    }

    fn new_rc_cell() -> RcCellWordsFrequencies {
        let wf = Rc::new(RefCell::new(WordsFrequencies::new()));
        Rc::clone(&wf).borrow_mut().weak_self = Rc::downgrade(&wf);
        wf
    }

    fn insert_word(&mut self, word: &String) {
        // println!("insert w {}", word);
        *self.words_frequencies.entry(word.clone()).or_insert(0) += 1;
    }

    fn print_to_string(&self, min_freq: i32, target_str: &mut String) {
        target_str.clear();

        let mut wv = self
            .words_frequencies
            .iter()
            .map(|el| (el.0.clone(), el.1.clone()))
            .collect::<Vec<_>>();

        wv.sort_by_key(|el| -el.1);

        target_str.clone_from(&wv.iter().filter(|el| el.1 >= min_freq).fold(
            String::new(),
            |mut st, el| {
                st.push_str(&format!("{} - {}\n", el.0, el.1));
                st
            },
        ));
    }

    fn register_events(&mut self, rc_ev_manager: &RcCellEventManager) {
        {
            // Load Stop words
            let self_rc = self.weak_self.upgrade().unwrap().clone();

            let insert_word_h: EventHandlerType = Rc::new(RefCell::new(
                //
                move |arg: Rc<dyn Any>| {
                    let word = arg.downcast_ref::<String>().unwrap();
                    Rc::clone(&self_rc).borrow_mut().insert_word(word);
                },
            ));

            Rc::clone(rc_ev_manager)
                .borrow_mut()
                .register_event(&MyEvent::OnValidWord, &insert_word_h);
        }

        {
            // Print frequencies to String
            let self_rc = self.weak_self.upgrade().unwrap().clone();

            let print_frequencies_h: EventHandlerType = Rc::new(RefCell::new(
                //
                move |arg: Rc<dyn Any>| {
                    let arg = arg.downcast_ref::<(i32, Rc<RefCell<String>>)>().unwrap();
                    let mut result_string = String::new();
                    let min_f = arg.0;

                    Rc::clone(&self_rc)
                        .borrow_mut()
                        .print_to_string(min_f, &mut result_string);

                    arg.1.borrow_mut().clone_from(&result_string);

                    // println!("{}", result_string);
                },
            ));

            Rc::clone(rc_ev_manager)
                .borrow_mut()
                .register_event(&MyEvent::OnPrint, &print_frequencies_h);
        }

        self.event_manager = Some(rc_ev_manager.clone());
    }
}

pub fn bulletin_board_test(file_name: &String, file_stop_w: &String) {
    let event_manager = EventManager::new_rc_cell();

    let data_storage = DataStorage::new_rc_cell();

    let stop_words = StopWords::new_rc_cell();

    let words_frequencies = WordsFrequencies::new_rc_cell();

    Rc::clone(&data_storage)
        .borrow_mut()
        .register_events(&event_manager);

    Rc::clone(&stop_words)
        .borrow_mut()
        .register_events(&event_manager);

    Rc::clone(&words_frequencies)
        .borrow_mut()
        .register_events(&event_manager);

    Rc::clone(&event_manager).borrow().emit(
        &MyEvent::OnLoad,
        &(Rc::new(file_name.clone()) as Rc<dyn Any>),
    );

    Rc::clone(&event_manager).borrow().emit(
        &MyEvent::OnLoadStopWords,
        &(Rc::new(file_stop_w.clone()) as Rc<dyn Any>),
    );

    Rc::clone(&event_manager)
        .borrow()
        .emit(&MyEvent::OnStart, &(Rc::new(()) as Rc<dyn Any>));

    let result_string = Rc::new(RefCell::new(String::new()));
    let arg_print = (5, Rc::clone(&result_string));

    Rc::clone(&event_manager)
        .borrow()
        .emit(&MyEvent::OnPrint, &(Rc::new(arg_print) as Rc<dyn Any>));

    println!("{}", Rc::clone(&result_string).borrow_mut());
}
