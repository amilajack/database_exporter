#![feature(test)]
extern crate serde_json;
extern crate test;
#[macro_use]
extern crate fake;
extern crate rusqlite;

mod generate_records;

fn main() {
    let conn = generate_records::create_conn();
    generate_records::create_table(&conn);
    generate_records::generate_records(&conn, 100_000);
    let result = generate_records::convert_records(&conn);
    println!("{}", result);
}
