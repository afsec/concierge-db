use rusqlite::{Result, NO_PARAMS};

use crate::api::StatusMessage;
use crate::api::{Coluna, ColunaData};
use crate::database::DbConnection;

pub fn insert_row(
    conn: DbConnection,
    table_name: String,
    colunas: Vec<Coluna>,
) -> Result<StatusMessage, StatusMessage> {
    if colunas.is_empty() {
        Err(StatusMessage::InvalidInput("data_str".to_string()))
    } else {
        let mut keys = String::new();
        let mut values = String::new();
        for idx in 0..colunas.len() {
            //Key
            {
                keys.push_str(&colunas[idx].column_name);
                if idx < (colunas.len() - 1) {
                    keys.push(',');
                }
            }
            // Value
            {
                let value = match &colunas[idx].data {
                    ColunaData::Integer(value) => format!("{}", value),
                    ColunaData::Text(value) => format!(r#""{}""#, value),
                    _ => "\"Not yet\"".to_string(),
                };
                values.push_str(&value);
                if idx < (colunas.len() - 1) {
                    values.push(',');
                }
            }
        }
        let query = format!("INSERT INTO {} ({}) VALUES ({})", table_name, keys, values);
        println!("{}", &query);
        match conn.execute(query.as_ref(), NO_PARAMS) {
            Ok(_) => Ok(StatusMessage::Saved),
            Err(error) => Err(StatusMessage::DatabaseError(error.to_string())),
        }
    }
}
