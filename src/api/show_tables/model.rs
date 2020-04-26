use rusqlite::{Connection, Result, NO_PARAMS};

pub fn show_tables() -> Result<Vec<String>> {
    const DATABASE_FILE: &str = "database.sqlite3";
    let conn = Connection::open(DATABASE_FILE).unwrap();

    let mut stmt = conn.prepare(
        r#"SELECT name FROM sqlite_master where type="table" AND name NOT LIKE "%sqlite%";"#,
    )?;
    let rows = stmt.query_map(NO_PARAMS, |row| row.get(0))?;

    let mut names = Vec::new();
    for name_result in rows {
        names.push(name_result?);
    }

    Ok(names)
}
