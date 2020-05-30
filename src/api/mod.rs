// pub mod insert_row;
pub mod read_all;
pub mod read_count;
pub mod show_tables;
// pub mod show_columns;
// pub mod update_field;

use serde::{Deserialize, Serialize};

// #[derive(Debug, Serialize, Deserialize)]
// pub enum StatusMessage {
//     Saved,
//     NotSaved,
//     DatabaseError(String),
//     SerdeError(String),
//     ConnectionError(String),
//     // NotProcessed(String),
//     InvalidInput(String),
//     MissingId,
//     MalformedId,
//     NotImplemented,
//     UnderMaintenance,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct BodyResponse {
//     pub status: StatusMessage,
// }

#[derive(Deserialize, Debug, Default)]
pub struct Table {
    name: Option<String>,
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
    //TODO: Implement SQLite `REAL` type
    // Real(f32),
    Boolean(bool),
    Datetime(i32),
    Null(()),
    Unknown(String),
}
