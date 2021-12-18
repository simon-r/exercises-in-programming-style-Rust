use regex::Regex;
use std::any::Any;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, LinkedList};
use std::fs;
use std::rc::Rc;
use std::rc::Weak;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum MyEvents {
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
    events_map: HashMap<MyEvents, LinkedList<EventHandlerType>>,
    weak_self: Weak<RefCell<Self>>,
}

type RcCellEventManagerT = Rc<RefCell<EventManager>>;

impl EventManager {
    fn new() -> Self {
        EventManager {
            events_map: HashMap::new(),
            weak_self: Weak::new(),
        }
    }

    fn new_rc_cell() -> RcCellEventManagerT {
        let evm = Rc::new(RefCell::new(EventManager::new()));
        Rc::clone(&evm).borrow_mut().weak_self = Rc::downgrade(&evm);
        evm
    }

    fn register_event(&mut self, ev: &MyEvents, h_fun: &EventHandlerType) {
        self.events_map
            .entry(*ev)
            .or_insert(LinkedList::new())
            .push_back(Rc::clone(h_fun));
    }

    fn emit(&mut self, ev: &MyEvents, arg: &EventArg) {
        if !self.events_map.contains_key(ev) {
            return;
        }

        let ev_list = self.events_map.get(ev).unwrap();

        for h_fun in ev_list {
            Rc::clone(h_fun).borrow_mut()(Rc::clone(arg));
        }
    }
}

/////////////////////////////////////////////////////////////////
struct DataStorage {
    text: String,
    weak_self: Weak<RefCell<Self>>,
}

impl DataStorage {
    fn new() -> Self {
        DataStorage {
            text: String::new(),
            weak_self: Weak::new(),
        }
    }

    fn new_rc_cell() -> Rc<RefCell<DataStorage>> {
        let ds = Rc::new(RefCell::new(DataStorage::new()));
        Rc::clone(&ds).borrow_mut().weak_self = Rc::downgrade(&ds);
        ds
    }

    fn register_event(&mut self, ev: &MyEvents, rc_ev_manager: &RcCellEventManagerT) {}

    fn load_data(&mut self, file_name: &String) {
        self.text = fs::read_to_string(file_name)
            .expect("Some error in reading data!")
            .to_lowercase();
    }
}

pub fn bulletin_board_test(file_name: &String, file_stop_w: &String) {
    let h: EventHandlerType = Rc::new(RefCell::new(|mut arg: Rc<dyn Any>| {
        let a = arg.downcast_ref::<i32>();
        println!("Bye {}", a.unwrap());
    }));

    let a = 0;
    h.clone().borrow_mut()(Rc::new(a) as Rc<dyn Any>);

    let mut ev_manager = EventManager::new();

    ev_manager.register_event(&MyEvents::OnLoad, &h);
    ev_manager.emit(&MyEvents::OnLoad, &(Rc::new(55) as Rc<dyn Any>));
    ev_manager.emit(&MyEvents::OnLoad, &(Rc::new(555) as Rc<dyn Any>));

    let event_manager = EventManager::new_rc_cell();

    let data_storage = DataStorage::new_rc_cell();

    //     let strong = Rc::new("hello".to_owned());
    // let weak = Rc::downgrade(&strong);
}
