use rusqlite::{params, Connection, Result};

struct MyDataBase {
    connection: Connection,
}

impl MyDataBase {
    fn new() -> Self {
        MyDataBase {
            connection: match Connection::open_in_memory() {
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
            "CREATE TABLE documents (id INTEGER PRIMARY KEY AUTOINCREMENT, name);
                 CREATE TABLE words (id, doc_id, value);
                 CREATE TABLE characters (id, word_id, value);",
            [],
        ) {
            Ok(_r) => {}
            Err(_e) => {
                assert!(false, "SqLite - Build Data Base Error");
            }
        }
    }
}

pub fn persistent_tables_test(file_name: &String, file_stop_w: &String) {
    let mut db = MyDataBase::new();
    db.build_data_base();
}
