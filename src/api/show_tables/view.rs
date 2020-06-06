use serde::Serialize;
use tide::{Body, Response, StatusCode};

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
    let mut response = Response::new(StatusCode::Ok);
    response.set_body(Body::from_json(&body_response).unwrap());
    response
}
