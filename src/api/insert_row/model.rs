use crate::api::{Coluna, ColunaData};

use crate::api::StatusMessage;

pub fn insert_row(table_name: String, data_str: String) -> Result<StatusMessage, StatusMessage> {
    let colunas: Vec<Coluna> = match serde_json::from_str(&data_str) {
        Ok(data) => data,
        Err(error) => return Err(StatusMessage::SerdeError(error.to_string()))
    };
    use rusqlite::{Connection, NO_PARAMS};
    if colunas.is_empty() {
        Err(StatusMessage::InvalidInput(data_str))
    } else {
        match Connection::open("database.sqlite3") {
            Ok(conn) => {
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

            Err(err) => {
                dbg!(&err);
                Err(StatusMessage::ConnectionError(err.to_string()))
            }
        }
    }
}
