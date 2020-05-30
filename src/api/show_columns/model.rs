use rusqlite::{Result, NO_PARAMS};

use crate::database::SqlitePooledConnection;

use super::ColumnInfo;

pub fn show_columns(conn: SqlitePooledConnection, table: String) -> Result<Vec<ColumnInfo>> {
    let query = format!("PRAGMA table_info('{}')", table);
    println!("{}", &query);
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
