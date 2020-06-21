use crate::api::Coluna;
use brickpack::{Body, Response, StatusCode};

pub fn read_all(rows: Vec<Vec<Coluna>>) -> Response {
    let mut response = Response::new(StatusCode::Ok);
    response.set_body(Body::from_json(&rows).unwrap());
    response
}
