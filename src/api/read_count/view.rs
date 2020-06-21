use serde::Serialize;
use brickpack::{Body, Response, StatusCode};

#[derive(Serialize, Debug)]
struct RowCount {
    rows: u16,
}

pub fn read_count(rows: u16) -> Response {
    let body_response = RowCount { rows };
    let mut response = Response::new(StatusCode::Ok);
    response.set_body(Body::from_json(&body_response).unwrap());
    response
}
