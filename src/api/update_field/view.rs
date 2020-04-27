use crate::api::{BodyResponse, StatusMessage};
use http_types::StatusCode;
use tide::Response;

pub fn update_field(message: StatusMessage) -> Response {
    let body_response = BodyResponse { status: message };
    Response::new(StatusCode::Ok)
        .body_json(&body_response)
        .unwrap()
}
