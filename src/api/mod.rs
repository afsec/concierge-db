pub mod insert_row;
pub mod read_all;
pub mod read_count;
pub mod show_columns;
pub mod show_tables;
// pub mod update_field;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum StatusMessage {
    InvalidInput(String),
    Saved,
    DatabaseError(String),
    // NotSaved,
    // SerdeError(String),
    // ConnectionError(String),
    // NotProcessed(String),
    // MissingId,
    // MalformedId,
    // NotImplemented,
    // UnderMaintenance,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BodyResponse {
    pub status: StatusMessage,
}

#[derive(Deserialize, Debug, Default)]
pub struct Table {
    table: Option<String>,
    columns: Option<Vec<Coluna>>
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
