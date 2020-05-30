use rusqlite::{Result, NO_PARAMS};

use crate::database::SqlitePooledConnection;

pub fn read_count(conn: SqlitePooledConnection, table: String) -> Result<u16, String> {
    //TODO: Validate input using regex ([a-z] || [A-Z] || [0-9] || [_] && ! ^[0-9])
    let query = format!("SELECT COUNT(*) FROM {};", table);
    tide::log::info!("SQL_QUERY: {}", &query);
    let row_count: u16 = conn.query_row(&query, NO_PARAMS, |r| r.get(0)).unwrap();
    Ok(row_count)
}
