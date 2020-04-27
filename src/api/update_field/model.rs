use crate::api::{Coluna, ColunaData};

use crate::api::StatusMessage;

pub fn update_field(
    table_name: String,
    colunas: Vec<Coluna>,
) -> Result<StatusMessage, StatusMessage> {
    use rusqlite::{Connection, NO_PARAMS};
    use std::convert::TryFrom;
    if (!colunas.is_empty()) && (colunas.len() > 1) {
        match Connection::open("database.sqlite3") {
            Ok(conn) => {
                let id: u16;
                let mut fields = String::new();

                // TODO: permitir apenas se o campo `id` for informado no request, que deverá sempre vir no primeiro objeto JSON do array.
                if colunas[0].column_name == "id" {
                    match colunas[0].data {
                        ColunaData::Integer(value) => {
                            match u16::try_from(value) {
                                Ok(x) => id = x,
                                Err(_) => return Err(StatusMessage::MalformedId),
                            };
                        }
                        _ => return Err(StatusMessage::InvalidInput("data_str".to_string())),
                    }
                } else {
                    return Err(StatusMessage::MissingId);
                }
                for idx in 1..colunas.len() {
                    {
                        //Key
                        fields.push_str(&colunas[idx].column_name);
                        fields.push('=');
                        // Value
                        let value = match &colunas[idx].data {
                            ColunaData::Integer(value) => format!("{}", value),
                            ColunaData::Text(value) => format!(r#""{}""#, value),
                            _ => "\"Not yet\"".to_string(),
                        };
                        fields.push_str(&value);
                        if idx < (colunas.len() - 1) {
                            fields.push(',');
                        }
                    }
                }
                let query = format!("UPDATE {} SET {} WHERE id={}", table_name, fields, id);
                // println!("{}", &query);
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
    } else {
        Err(StatusMessage::InvalidInput("data_str".to_string()))
    }
}
