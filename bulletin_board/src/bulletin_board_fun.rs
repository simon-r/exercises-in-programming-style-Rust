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
            Rc::clone(&h_fun).borrow()(Rc::clone(arg));
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
                move |mut arg: Rc<dyn Any>| {
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
                move |mut _arg: Rc<dyn Any>| {
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
            println!("word: {}", &word);

            Rc::clone(&evm)
                .borrow()
                .emit(&MyEvent::OnWord, &(Rc::new(word.clone()) as Rc<dyn Any>));
        }
    }
}

pub fn bulletin_board_test(file_name: &String, file_stop_w: &String) {
    // let h: EventHandlerType = Rc::new(RefCell::new(|mut arg: Rc<dyn Any>| {
    //     let a = arg.downcast_ref::<i32>();
    //     println!("Bye {}", a.unwrap());
    // }));

    // let a = 0;
    // h.clone().borrow_mut()(Rc::new(a) as Rc<dyn Any>);

    // let mut ev_manager = EventManager::new();
    // //
    // ev_manager.register_event(&MyEvent::OnLoad, &h);
    // ev_manager.register_event(&MyEvent::OnLoad, &h);
    // ev_manager.register_event(&MyEvent::OnLoad, &h);

    // ev_manager.emit(&MyEvent::OnLoad, &(Rc::new(55) as Rc<dyn Any>));
    // ev_manager.emit(&MyEvent::OnLoad, &(Rc::new(555) as Rc<dyn Any>));

    let event_manager = EventManager::new_rc_cell();

    let data_storage = DataStorage::new_rc_cell();

    Rc::clone(&data_storage)
        .borrow_mut()
        .register_events(&event_manager);

    Rc::clone(&event_manager).borrow().emit(
        &MyEvent::OnLoad,
        &(Rc::new(file_name.clone()) as Rc<dyn Any>),
    );

    Rc::clone(&event_manager)
        .borrow()
        .emit(&MyEvent::OnStart, &(Rc::new(()) as Rc<dyn Any>));
}
