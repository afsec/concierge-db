use http_types::StatusCode;
use tide::Response;
use crate::api::Coluna;

pub fn read_all(rows: Vec<Vec<Coluna>>) -> Response {
    Response::new(StatusCode::Ok)
        .body_json(&rows)
        .unwrap()
}
