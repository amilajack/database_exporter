extern crate rusqlite;
#[macro_use]
extern crate fake;

mod generate_records;

fn main() {
    let conn = generate_records::create_conn();
    generate_records::create_table(&conn);
    generate_records::generate_records(&conn);
}
