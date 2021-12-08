use regex::Regex;
use std::any::Any;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, LinkedList, VecDeque};
use std::fs;
use std::rc::Rc;

type HM = HashMap<String, Rc<dyn Any>>;
type ShHM = Rc<RefCell<HM>>;

///////////////////////////////////////////////
/// Text manager
///////////////////////////////////////////////
type RcFunString = Rc<dyn Fn(&String) -> ()>;
type RcFunExtractWords = Rc<dyn Fn() -> Vec<String>>;

fn make_text_manager() -> ShHM {
    let text_manager: ShHM = ShHM::new(RefCell::new(HM::new()));

    {
        let text_manager_clone = Rc::clone(&text_manager);

        let read_text: RcFunString = Rc::new(move |file_name: &String| {
            let text = Rc::new(
                fs::read_to_string(file_name)
                    .expect("Error in real data")
                    .to_lowercase(),
            );
            // println!("{}", text);
            text_manager_clone
                .borrow_mut()
                .insert("text".to_string(), text);
        });

        text_manager
            .borrow_mut()
            .insert("read_text".to_string(), Rc::new(read_text));
    }

    {
        let text_manager_clone = Rc::clone(&text_manager);

        let extract_words: RcFunExtractWords = Rc::new(move || -> Vec<String> {
            let text = text_manager_clone
                .borrow_mut()
                .get(&"text".to_string())
                .unwrap()
                .downcast_ref::<String>()
                .unwrap()
                .to_lowercase();

            Regex::new(r"[\W_]+")
                .unwrap()
                .replace_all(&text, " ")
                .to_string()
                .split_whitespace()
                .fold(Vec::new(), |mut vs, el| {
                    vs.push(el.to_string());
                    vs
                })
        });

        text_manager
            .borrow_mut()
            .insert("extract_words".to_string(), Rc::new(extract_words));
    }

    text_manager
}

/////////////////////////////////////////////////////////////
///
type RcFunReadStopWords = Rc<dyn Fn(&String) -> ()>;
type RcFunIsStopWord = Rc<dyn Fn(&String) -> bool>;

fn make_stop_words_manager() -> ShHM {
    let stop_word_manager: ShHM = ShHM::new(RefCell::new(HM::new()));

    {
        let stop_word_manager_clone = Rc::clone(&stop_word_manager);

        let read_stop_words: RcFunReadStopWords = Rc::new(move |file_name: &String| {
            let stop_words = fs::read_to_string(file_name)
                .expect("something went wrong in reading stop words")
                .split(",")
                .fold(HashSet::new(), |mut hs, el| {
                    hs.insert(String::from(el));
                    hs
                });

            stop_word_manager_clone
                .borrow_mut()
                .insert("stop_words".to_string(), Rc::new(stop_words));
        });
        stop_word_manager
            .borrow_mut()
            .insert("read_stop_words".to_string(), Rc::new(read_stop_words));
    }

    {
        let stop_word_manager_clone = Rc::clone(&stop_word_manager);

        let is_stop_word: RcFunIsStopWord = Rc::new(move |word: &String| -> bool {
            stop_word_manager_clone
                .borrow_mut()
                .get("stop_words")
                .unwrap()
                .downcast_ref::<HashSet<String>>()
                .unwrap()
                .contains(word)
        });
        stop_word_manager
            .borrow_mut()
            .insert("is_stop_word".to_string(), Rc::new(is_stop_word));
    }

    stop_word_manager
}

pub fn closed_maps_test(file_name: &String, file_stop_w: &String) {
    let text_manager = make_text_manager();
    let stop_word_manager = make_stop_words_manager();

    let f_read_stop_words = stop_word_manager
        .borrow_mut()
        .get_mut(&"read_stop_words".to_string())
        .unwrap()
        .clone();
    f_read_stop_words
        .downcast_ref::<RcFunReadStopWords>()
        .unwrap()
        .clone()(file_stop_w);

    let f_read_text = text_manager
        .borrow_mut()
        .get_mut(&"read_text".to_string())
        .unwrap()
        .clone();

    f_read_text.downcast_ref::<RcFunString>().unwrap().clone()(file_name);

    let extract_words_f = text_manager
        .borrow_mut()
        .get_mut(&"extract_words".to_string())
        .unwrap()
        .clone();

    let vec_w = extract_words_f.downcast_ref::<RcFunExtractWords>().unwrap()();

    let f_is_stop_word = stop_word_manager
        .borrow_mut()
        .get_mut(&"is_stop_word".to_string())
        .unwrap()
        .clone();

    for el in vec_w {
        if !f_is_stop_word
            .downcast_ref::<RcFunIsStopWord>()
            .unwrap()
            .clone()(&el)
        {
            println!("{}", el);
        }
    }
}
