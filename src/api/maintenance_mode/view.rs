use http_types::StatusCode;
use tide::Response;

use crate::api::{BodyResponse, StatusMessage};

pub fn maintenance_mode(message: StatusMessage) -> Response {
    let body_response = BodyResponse {
        status: message,
    };

    Response::new(StatusCode::Ok)
        .body_json(&body_response)
        .unwrap()
}
