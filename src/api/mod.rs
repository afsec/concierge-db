// pub mod update_field;

pub mod insert_row;
pub mod read_all;
pub mod read_count;
pub mod show_columns;
pub mod show_tables;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum StatusMessage {
    Saved,
    NotSaved,
    DatabaseError(String),
    SerdeError(String),
    ConnectionError(String),
    // NotProcessed(String),
    InvalidInput(String),
    MissingId,
    MalformedId,
    NotImplemented,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BodyResponse {
    pub status: StatusMessage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Coluna {
    column_name: String,
    data: ColunaData,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ColunaData {
    Text(String),
    Integer(i32),
    // Real(f32),
    Boolean(bool),
    Datetime(i32),
    Null(()),
    Unknown(String),
}

// mod converters {
//     use rocket::Data;

//     // ### Converte o tipo Data do Rocket para um tipo String
//     pub fn data_to_string(data: Data) -> String {
//         // ##### Conversão da entrada via Body para uma String
//         use std::io::Read;
//         let mut respostas = String::new();
//         if let Err(e) = data.open().read_to_string(&mut respostas) {
//             return e.to_string();
//         };
//         respostas
//     }
// }
