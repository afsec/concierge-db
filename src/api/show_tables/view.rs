use serde::Serialize;
use tide::{Response,StatusCode};

#[derive(Serialize, Debug)]
struct ShowTables {
    tables: Option<Vec<String>>,
}

pub fn show_tables(rows: Vec<String>) -> Response {
    let body_response = if !rows.is_empty() {
        ShowTables { tables: Some(rows) }
    } else {
        ShowTables { tables: None }
    };
    Response::new(StatusCode::Ok)
        .body_json(&body_response)
        .unwrap()
}
