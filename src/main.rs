extern crate rusqlite;

use rusqlite::Connection;
use std::collections::HashMap;

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>
}

fn main() {
    let conn = Connection::open_in_memory().unwrap();

    conn.execute("CREATE TABLE person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  data            BLOB
                  )", &[]).unwrap();
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None
    };
    // fields are known at runtime
    let fields = [
        String::from("id"),
        String::from("name"),
        String::from("data"),
    ];
    // Query should also be known at runtime
    conn.execute("INSERT INTO person (name, data)
                  VALUES (?1, ?2, ?3)",
                 &[&me.name, &me.data]).unwrap();

    let mut stmt = conn.prepare("SELECT id, name, data FROM person").unwrap();
    let person_iter = stmt.query_map(&[], |row| {
        for item in fields.iter() {
            let mut map: HashMap<String, String> = HashMap::new();
            map.insert(item.to_string(), row.get(0));
        }
    }).unwrap();

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }
}
