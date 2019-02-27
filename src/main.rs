#![feature(test)]
extern crate serde_json;
extern crate test as cargo_test;
#[macro_use]
extern crate fake;
extern crate rusqlite;

use rusqlite::Connection;
mod export;
mod test;

fn create_conn() -> Connection {
    Connection::open_in_memory().unwrap()
}

fn main() {
    let conn = create_conn();
    test::create_table(&conn);
    test::seed_db(&conn, 100_000);
    let result = export::export("users", Some(&conn));
    println!("{}", result);
}
