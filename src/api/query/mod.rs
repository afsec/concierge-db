mod model;
mod view;

use brickpack::{
    build_presenter,
    endpoint::{Endpoint, Name, Outcome, Presenter},
};
use brickpack_derive::{Endpoint, Outcome};
use serde::{Deserialize, Serialize};

#[derive(Debug, Outcome, Serialize)]
struct InternalMessage(Vec<Vec<Column>>);

#[derive(Debug, Endpoint)]
#[endpoint_name = "query"]
struct QueryDatabase;

build_presenter!(QueryDatabase, InternalMessage);

// pub struct Table {
//     table: Option<String>,
//     columns: Option<Vec<Column>>
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct Column {
    column_name: String,
    data: ColumnData,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ColumnData {
    Text(String),
    Integer(i32),
    //TODO: Implement SQLite `REAL` type
    // Real(f32),
    Boolean(bool),
    Datetime(i32),
    Null(()),
    Unknown(String),
}