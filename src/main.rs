#![feature(test)]
extern crate serde_json;
extern crate test as cargo_test;
#[macro_use]
extern crate fake;
extern crate rusqlite;

mod export;
mod test;

fn main() {
    let conn = export::create_conn();
    test::create_table(&conn);
    test::seed_db(&conn, 100_000);
    let result = export::export("users", &conn);
    println!("{}", result);
}
