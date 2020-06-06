use super::ColumnInfo;
use tide::{Body, Response, StatusCode};

pub fn show_columns(columns: Vec<ColumnInfo>) -> Response {
    let mut response = Response::new(StatusCode::Ok);
    response.set_body(Body::from_json(&columns).unwrap());
    response
}
