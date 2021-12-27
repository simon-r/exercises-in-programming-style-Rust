use regex::Regex;
use sqlite::{Connection};
use std::fs;

struct MyDataBase {
    connection: Connection,
}

impl MyDataBase {
    fn new() -> Self {
        MyDataBase {
            connection: match sqlite::open(":memory:") {
                Ok(c) => c,
                Err(_e) => {
                    assert!(false, "SqLite - Connection error");
                    Connection::open("fake").ok().unwrap()
                }
            },
        }
    }

    fn build_data_base(&mut self) {
        match self.connection.execute(
            "CREATE TABLE words (
                        Id integer primary key autoincrement , 
                        word varchar(60));
                      
                      CREATE TABLE stop_words (
                        Id integer primary key autoincrement , 
                        stop_word varchar(60));",
        ) {
            Ok(_r) => {}
            Err(_e) => {
                assert!(false, "SqLite - Build Data Base Error");
            }
        }
    }

    fn fill_table_words(&mut self, file_name: &String) {
        let vec_w = Regex::new(r"[\W_]+")
            .unwrap()
            .replace_all(
                &fs::read_to_string(file_name).expect("Something went wrong with the in Text"),
                " ",
            )
            .to_lowercase()
            .split(" ")
            .map(|el| String::from(el))
            .collect::<Vec<_>>();

        for word in vec_w {
            match self
                .connection
                .execute(format!("INSERT into words (word) values (\"{}\")", word))
            {
                Ok(_r) => {}
                Err(_e) => {
                    assert!(false, "SqLite - error in inserting a word");
                }
            }
        }
    }

    fn fill_table_stop_words(&mut self, file_stop_w: &String) {
        let vec_sw = fs::read_to_string(file_stop_w)
            .expect("An errors occurred in reading stop words")
            .split(",")
            .map(|el| String::from(el))
            .collect::<Vec<_>>();

        for stopw in vec_sw {
            match self.connection.execute(format!(
                "INSERT into stop_words (stop_word) values (\"{}\")",
                stopw
            )) {
                Ok(_r) => {}
                Err(_e) => {
                    assert!(false, "SqLite - error in inserting a stop word");
                }
            }
        }
    }

    fn query_top_words(&mut self, top: u32) {
        let query = format!(
            "SELECT word, count(word) from words 
                WHERE word not in 
                (
                    SELECT stop_word FROM stop_words
                )
                AND LENGTH(word) > 1
                GROUP BY word
                HAVING COUNT(word) > {}
                ORDER BY COUNT(word) DESC;
            ",
            top
        );

        let mut cursor = self.connection.prepare(query).unwrap().into_cursor();

        while let Some(row) = cursor.next().unwrap() {
            println!(
                "{} - {}",
                row[0].as_string().unwrap(),
                row[1].as_integer().unwrap()
            );
        }
    }
}

pub fn persistent_tables_test(file_name: &String, file_stop_w: &String) {
    let mut db = MyDataBase::new();
    db.build_data_base();
    db.fill_table_words(file_name);
    db.fill_table_stop_words(file_stop_w);
    db.query_top_words(4);
}
