use super::{InternalMessage, ShowTables};
use brickpack::endpoint::{Model, Name};
use rusqlite::{Connection, NO_PARAMS};
use tide::{Error as TideError, StatusCode};

impl Model<InternalMessage> for ShowTables {
    fn model(&self, request_body: String) -> Result<InternalMessage, TideError> {
        if !request_body.is_empty() {
            let conn = Connection::open("database.sqlite3")?;

            let query =
                "SELECT name FROM sqlite_master where type='table' AND name NOT LIKE '%sqlite%';";

            tide::log::warn!("Endpoint [{}]: {}", self.name(), &query);

            let mut stmt = conn.prepare(query)?;

            let rows = stmt.query_map(NO_PARAMS, |row| row.get(0))?;

            let mut tables: Vec<String> = Vec::new();
            for row_result in rows {
                tables.push(row_result?)
            }
            Ok(InternalMessage(tables))
        } else {
            Err(TideError::from_str(
                StatusCode::BadRequest,
                "Body is empty".to_owned(),
            ))
        }
    }
}
