use rusqlite::{Connection, NO_PARAMS};

pub fn read_count(table: String) -> Result<u16, String> {
    const DATABASE_FILE: &str = "database.sqlite3";
    let conn = Connection::open(DATABASE_FILE).unwrap();
    let query = format!("SELECT COUNT(*) FROM {};", table);
    let row_count: u16 = conn.query_row(&query, NO_PARAMS, |r| r.get(0)).unwrap();
    Ok(row_count)
}
