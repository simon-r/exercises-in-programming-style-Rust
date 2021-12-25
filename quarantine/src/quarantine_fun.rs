use regex::Regex;
use std::any::Any;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, LinkedList};
use std::fs;
use std::rc::Rc;

type ValueType = Rc<RefCell<dyn Any>>;

type CallableArgType = Rc<RefCell<dyn Fn(ValueType) -> ValueType>>;
type CallableType = Rc<RefCell<dyn Fn() -> ValueType>>;

macro_rules! make_rc_refcell {
    ($value:expr) => {
        Rc::new(RefCell::new($value))
    };
}

macro_rules! make_callable_arg {
    ($fun_name:ident, $type_arg:ty) => {
        make_rc_refcell!(
            //
            move |arg: ValueType| {
                let res = $fun_name(arg.clone().borrow().downcast_ref::<$type_arg>().unwrap());
                make_rc_refcell!(res) as ValueType
            }
        ) as CallableArgType
    };
}

macro_rules! make_callable {
    ($fun_name:ident) => {
        make_rc_refcell!(
            //
            move || {
                let res = $fun_name();
                make_rc_refcell!(res) as ValueType
            }
        ) as CallableType
    };
}

fn guard_callable(vt: &ValueType) -> ValueType {
    match vt.clone().borrow_mut().downcast_ref::<CallableType>() {
        Some(rc_fun) => {
            return rc_fun.clone().borrow()();
        }
        None => {
            return vt.clone();
        }
    }
}

struct TFQuarantine {
    list_q: LinkedList<CallableArgType>,
    value_c: ValueType,
}

impl TFQuarantine {
    fn new() -> Self {
        TFQuarantine {
            list_q: LinkedList::new(),
            value_c: make_rc_refcell!(Result::<bool, bool>::Err(false)),
        }
    }

    fn push_fun(&mut self, fun: &CallableArgType) -> &mut Self {
        self.list_q.push_back(fun.clone());
        self
    }

    fn run(&mut self, start_arg: ValueType) -> ValueType {
        self.value_c = start_arg.clone();
        for el in &self.list_q {
            self.value_c = guard_callable(&el.clone().borrow_mut()(self.value_c.clone()));
        }

        self.value_c.clone()
    }
}

fn get_input(file_name: &String) -> CallableType {
    let file_name_clone = file_name.clone();

    let fun = move || {
        // println!("get_input");
        fs::read_to_string(&file_name_clone).expect("Some error in data file")
    };

    make_callable!(fun)
}

fn extract_words(text: &String) -> CallableType {
    let text_clone = text.clone();
    let fun = move || {
        // println!("extract_words {}", text_clone);
        Regex::new(r"[\W_]+")
            .unwrap()
            .replace_all(&text_clone.to_lowercase(), " ")
            .split(" ")
            .into_iter()
            .map(|el| String::from(el))
            .collect::<Vec<String>>()
    };

    make_callable!(fun)
}

fn remove_stop_words(file_stop_w: &String) -> CallableArgType {
    let stop_words = fs::read_to_string(file_stop_w)
        .expect("Something wrong with stop words")
        .split(",")
        .map(|el| String::from(el))
        .collect::<HashSet<String>>();

    let fun = move |vs: &Vec<String>| -> Vec<String> {
        // println!("romeve stop W");
        vs.iter()
            .filter(|el| !stop_words.contains(*el) && el.len() > 1)
            .map(|el| String::from(el))
            .collect::<Vec<String>>()
    };

    make_callable_arg!(fun, Vec<String>)
}

fn frequencies(words_vec: &Vec<String>) -> HashMap<String, i32> {
    // println!("frequencies");
    words_vec.iter().fold(HashMap::new(), |mut hm, el| {
        *hm.entry(el.clone()).or_insert(0) += 1;
        hm
    })
}

fn sort(hm: &HashMap<String, i32>) -> Vec<(String, i32)> {
    // println!("sort");
    let mut vf = hm
        .into_iter()
        .map(|el| (el.0.clone(), el.1.clone()))
        .collect::<Vec<_>>();
    vf.sort_by_key(|el| -el.1);
    vf
}

fn print_top_to_string(top: i32) -> CallableArgType {
    let fun = move |vf: &Vec<(String, i32)>| -> String {
        // println!("print_top_to_string");
        vf.into_iter().fold(String::new(), |mut st, el| {
            if el.1 > top {
                st.push_str(&format!("{} - {}\n", el.0, el.1));
            };
            st
        })
    };

    make_callable_arg!(fun, Vec<(String, i32)>)
}

pub fn quarantine_test(file_name: &String, file_stop_w: &String) {
    let mut tfq = TFQuarantine::new();

    let call_get_input = make_callable_arg!(get_input, String);
    let call_extract_words = make_callable_arg!(extract_words, String);
    let call_remove_stop_words = remove_stop_words(file_stop_w);
    let call_frequencies = make_callable_arg!(frequencies, Vec<String>);
    let call_sort = make_callable_arg!(sort, HashMap<String, i32>);
    let call_top = print_top_to_string(4);

    let res = tfq
        .push_fun(&call_get_input) //
        .push_fun(&call_extract_words)
        .push_fun(&call_remove_stop_words)
        .push_fun(&call_frequencies)
        .push_fun(&call_sort)
        .push_fun(&call_top)
        .run(make_rc_refcell!(String::from(file_name)))
        .borrow()
        .downcast_ref::<String>()
        .unwrap()
        .clone();

    println!("{}", res);
}
