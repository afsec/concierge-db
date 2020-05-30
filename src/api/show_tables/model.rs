use rusqlite::{Result, NO_PARAMS};

use crate::database::SqlitePooledConnection;

pub fn show_tables(conn: SqlitePooledConnection) -> Result<Vec<String>> {
    let query =
        r#"SELECT name FROM sqlite_master where type="table" AND name NOT LIKE "%sqlite%";"#;
    println!("{}", &query);
    let mut stmt = conn.prepare(query)?;
    let rows = stmt.query_map(NO_PARAMS, |row| row.get(0))?;

    let mut names = Vec::new();
    for name_result in rows {
        names.push(name_result?);
    }

    Ok(names)
}
