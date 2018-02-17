#![feature(test)]
extern crate test;
#[macro_use]
extern crate fake;
extern crate rusqlite;

mod generate_records;

fn main() {
    let conn = generate_records::create_conn();
    generate_records::create_table(&conn);
    generate_records::generate_records(&conn);
}
