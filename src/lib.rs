#![feature(test)]
extern crate test;
extern crate serde_json;
#[macro_use]
extern crate fake;
extern crate rusqlite;

mod generate_records;

pub use generate_records::convert_records;
