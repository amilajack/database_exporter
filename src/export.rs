use rusqlite::types::Value;
use rusqlite::{Connection, Error};
use serde_json;

pub fn export(table: &str, conn: &Connection) -> String {
    let stmt = format!("SELECT * FROM {}", table);
    let mut stmt = conn.prepare(stmt.as_str()).unwrap();
    let result: Result<Vec<Vec<String>>, Error> = stmt
        .query_map(&[], |_row| {
            let mut vec = Vec::new();
            let column_index = _row.column_count();
            for i in 1..column_index {
                let val: String = match _row.get(i) {
                    Value::Null => "NULL".to_string(),
                    Value::Integer(e) => e.to_string(),
                    Value::Real(e) => e.to_string(),
                    Value::Text(e) => e.to_string(),
                    Value::Blob(_e) => "BLOB".to_string(),
                };
                vec.push(val);
            }
            vec
        })
        .unwrap()
        .collect();

    serde_json::to_string(&result.unwrap()).unwrap()
}

pub fn create_conn() -> Connection {
    Connection::open_in_memory().unwrap()
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;

    fn create_table(conn: &Connection) {
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


    fn get_column_names(conn: &Connection) -> Vec<String> {
        let mut columns_query_stmt = conn.prepare("PRAGMA table_info(users)").unwrap();
        let result: Result<Vec<String>, Error> = columns_query_stmt
            .query_map(&[], |_row| _row.get(1))
            .unwrap()
            .collect();
        result.unwrap()
    }

    #[test]
    fn test_column_names() {
        let conn = create_conn();
        get_column_names(&conn);
    }

    #[bench]
    fn bench_export(bench: &mut test::Bencher) {
        let conn = create_conn();
        create_table(&conn);
        bench.iter(|| export("users", &conn));
    }
}
