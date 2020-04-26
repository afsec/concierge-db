use http_types::StatusCode;
use serde::Serialize;
use tide::Response;

#[derive(Serialize, Debug)]
struct RowCount {
    rows: u16,
}

pub fn read_count(rows: u16) -> Response {
    let body_response = RowCount { rows };
    Response::new(StatusCode::Ok)
        .body_json(&body_response)
        .unwrap()
}
