use regex::Regex;
use std::any::Any;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, LinkedList};
use std::fs;
use std::rc::Rc;
use std::rc::Weak;

type CallableType = Rc<RefCell<dyn Fn()>>;
type ValueType = Rc<RefCell<dyn Any>>;

enum CallableVal {
    call(CallableType),
    val(ValueType),
}

struct TFQuarantine {
    list_q : LinkedList<CallableType>,
}

pub fn quarantine_test() {}
