use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;

fn read_and_filter_words(
    stop_w_file: &String,
    path_to_file: &String,
) -> Result<Vec<String>, String> {
    let stop_words = fs::read_to_string(stop_w_file);

    match &stop_words {
        Ok(_s) => {}
        Err(_s) => {
            return Err(String::from(format!(
                "Error in reading SW file: {}",
                stop_w_file
            )))
        }
    };

    let hs = stop_words
        .ok()
        .unwrap()
        .split(",")
        .into_iter()
        .map(|el| String::from(el))
        .collect::<HashSet<String>>();

    let text = fs::read_to_string(path_to_file);

    match &text {
        Ok(_s) => {}
        Err(_s) => {
            return Err(String::from(format!(
                "Error in reading data file: {}",
                path_to_file
            )))
        }
    };

    let text_str = text.ok().unwrap().to_lowercase();

    let word_vec = Regex::new(r"[\W_]+")
        .unwrap()
        .replace_all(&text_str, " ")
        .to_string()
        .split_whitespace()
        .map(|el| String::from(el))
        .filter(|el| !hs.contains(el) && el.len() > 1)
        .collect::<Vec<String>>();

    Ok(word_vec)
}

fn frequencies_to_string(vec_str: &Vec<String>) -> String {
    let mut vs = vec_str
        .iter()
        .fold(HashMap::new(), |mut hm, el| {
            *hm.entry(el.clone()).or_insert(0) += 1;
            hm
        })
        .into_iter()
        .map(|el| el)
        .collect::<Vec<_>>();
    vs.sort_by_key(|el| -el.1);

    vs.iter().fold(String::new(), |mut st, el| {
        if el.1 > 4 {
            st.push_str(&format!("{} - {}\n", el.0, el.1));
        }
        st
    })
}

//////////////////////////////////////////////////////////////////
pub fn constructivist_test(file_name: &String, file_stop_w: &String) {
    let res2 = read_and_filter_words(file_stop_w, file_name);

    match &res2 {
        Ok(s) => {
            let res = frequencies_to_string(s);
            println!("{}", res);
        }
        Err(s) => {
            println!("Error in opening file: {}", s);
        }
    }

    let res1 = read_and_filter_words(&String::from("fake file"), &String::from("fake file"));

    match &res1 {
        Ok(_s) => {}
        Err(s) => {
            println!("Error in opening file: {}", s)
        }
    }
}
