pub mod model;
pub mod presenter;
mod view;

use serde::Serialize;

#[derive(Serialize, Debug)]

pub struct ColumnInfo {
    pub column_name: String,
    pub column_type: String
}
