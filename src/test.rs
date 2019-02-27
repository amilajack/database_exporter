use rusqlite::Connection;

pub fn create_table(conn: &Connection) {
    let _ = conn.execute(
        "
        CREATE TABLE users (
            id              INTEGER PRIMARY KEY,
            name            TEXT NOT NULL,
            data            BLOB
        )",
        &[],
    );
}

// Could use std::any::Any; to implement 'dynamic types'
pub fn seed_db(conn: &Connection, record_count: i32) {
    for _ in 0..record_count {
        conn.execute(
            "INSERT INTO users (name, data)
            VALUES (?1, ?2)",
            &[&fake!(Name.name), &12],
        )
        .unwrap();
    }
}
