use super::ColumnInfo;

use rusqlite::{Connection, Result, NO_PARAMS};

// fn show_tables(conn: &Connection) -> Result<Vec<String>> {
pub fn show_columns(table: String) -> Result<Vec<ColumnInfo>> {
    const DATABASE_FILE: &str = "database.sqlite3";
    let conn = Connection::open(DATABASE_FILE).unwrap();
    let query = format!("PRAGMA table_info('{}')", table);
    let mut stmt = conn.prepare(&query)?;
    let rows = stmt.query_map(NO_PARAMS, |row| {
        Ok(ColumnInfo {
            column_name: row.get(1).unwrap(),
            column_type: row.get(2).unwrap(),
        })
    })?;

    let mut names = Vec::new();
    for name_result in rows {
        names.push(name_result?);
    }

    Ok(names)
}
