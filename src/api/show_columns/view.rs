use tide::{Response,StatusCode};
use super::ColumnInfo;

pub fn show_columns(columns: Vec<ColumnInfo>) -> Response {
    Response::new(StatusCode::Ok)
    .body_json(&columns)
    .unwrap()
}
