use super::{Column, ColumnData, QueryDatabase, InternalMessage};

use brickpack::endpoint::{Model, Name};

use rusqlite::{Connection, Result as RusqliteResult, NO_PARAMS};
use serde::Deserialize;
use serde_json::from_str as serde_json_from_str;
use tide::Error as TideError;
#[derive(Debug, Deserialize)]
struct RequestQuery {
    query: String,
}

impl Model<InternalMessage> for QueryDatabase {
    fn model(&self, request_body: String) -> Result<InternalMessage, TideError> {
        let conn = Connection::open("database.sqlite3")?;
        let request_query: RequestQuery = serde_json_from_str(&request_body)?;
        let query = request_query.query;
        tide::log::warn!("Endpoint [{}]: {}", self.name(), &query);

        let data = query_database(conn, query)?;
        Ok(InternalMessage(data))
    }
}

pub fn query_database(conn: Connection, query: String) -> RusqliteResult<Vec<Vec<Column>>> {
    tide::log::info!("SQL_QUERY: {}", &query);
    let mut stmt = conn.prepare(&query)?;

    let rows = stmt.query_map(NO_PARAMS, |row| {
        let num_cols = row.column_count();
        let mut retrieved_row: Vec<Column> = Vec::new();
        retrieved_row.reserve(num_cols);
        let column = &row.columns();
        // TODO: for i in 0..num_cols {
        // TODO: column.iter().enumerate().take(num_cols)
        for idx in 0..num_cols {
            match column[idx].decl_type().unwrap() {
                "INTEGER" => {
                    let result: RusqliteResult<i32> = row.get(idx);
                    match result {
                        Ok(integer) => {
                            retrieved_row.push(Column {
                                column_name: column[idx].name().to_string(),
                                data: ColumnData::Integer(integer),
                            });
                        }
                        Err(_) => {
                            retrieved_row.push(Column {
                                column_name: column[idx].name().to_string(),
                                data: ColumnData::Null(()),
                            });
                        }
                    }
                }
                "TEXT" => {
                    let result: RusqliteResult<String> = row.get(idx);
                    match result {
                        Ok(text) => {
                            retrieved_row.push(Column {
                                column_name: column[idx].name().to_string(),
                                data: ColumnData::Text(text),
                            });
                        }
                        Err(_) => {
                            retrieved_row.push(Column {
                                column_name: column[idx].name().to_string(),
                                data: ColumnData::Null(()),
                            });
                        }
                    }
                }
                "BOOLEAN" => {
                    let result: RusqliteResult<bool> = row.get(idx);
                    match result {
                        Ok(boolean) => {
                            retrieved_row.push(Column {
                                column_name: column[idx].name().to_string(),
                                data: ColumnData::Boolean(boolean),
                            });
                        }
                        Err(_) => {
                            retrieved_row.push(Column {
                                column_name: column[idx].name().to_string(),
                                data: ColumnData::Null(()),
                            });
                        }
                    }
                }
                "DATETIME" => {
                    let result: RusqliteResult<i32> = row.get(idx);
                    match result {
                        Ok(epoch) => {
                            let datetime: i32 = if epoch > 0 { epoch } else { 0 };
                            retrieved_row.push(Column {
                                column_name: column[idx].name().to_string(),
                                data: ColumnData::Datetime(datetime),
                            });
                        }
                        Err(_) => {
                            retrieved_row.push(Column {
                                column_name: column[idx].name().to_string(),
                                data: ColumnData::Null(()),
                            });
                        }
                    }
                }
                _ => {
                    // TODO: Implement types: REAL, NULL, BLOB
                    tide::log::error!("TypeName unknown");
                    tide::log::error!("{:?}", column[idx].decl_type());
                    let data: String = row.get(idx).unwrap();
                    tide::log::debug!("{}", &data);
                    retrieved_row.push(Column {
                        column_name: column[idx].name().to_string(),
                        data: ColumnData::Unknown(data),
                    });
                }
            }
        }
        Ok(retrieved_row)
    })?;
    let mut retrieved_rows = Vec::new();
    for name_result in rows {
        retrieved_rows.push(name_result?);
    }

    Ok(retrieved_rows)
}
