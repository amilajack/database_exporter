use rusqlite::{Connection, Error};

pub fn generate_records(conn: &Connection) {
    let mut stmt = try!(conn.prepare("SELECT * FROM users"));
    let result = stmt.query_map(&[], |row| {
        row.get(0)
    });
    // let result: Result<i64, Error> = conn.query_row("SELECT * FROM users", &[], |r| r.get(1));

    match result {
        Ok(res) => {
            println!("{} exists", res);
        }
        Err(er) => {
            println!("{}", er);
        },
    }
    for i in 0..12 {
        conn.execute("INSERT INTO users (name, data)
            VALUES (?1, ?2)",
            &[&fake!(Name.name), &"bar"]).unwrap();
    }
}

pub fn create_conn() -> Connection {
    Connection::open("./db.sqlite").unwrap()
}

pub fn create_table(conn: &Connection) {
    let foo = conn.execute("
        CREATE TABLE users (
            id              INTEGER PRIMARY KEY,
            name            TEXT NOT NULL,
            data            BLOB
        )", &[]
    );

    match foo {
        Ok(f) => {}
        Err(e) => {}
    }
}
