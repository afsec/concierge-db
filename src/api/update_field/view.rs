use brickpack::{Body, Response, StatusCode};

use crate::api::{BodyResponse, StatusMessage};

pub fn update_field(model_result: Result<StatusMessage, StatusMessage>) -> Response {
    match model_result {
        Ok(message) => {
            let body_response = BodyResponse { status: message };
            let mut response = Response::new(StatusCode::Ok);
            response.set_body(Body::from_json(&body_response).unwrap());
            response
        }
        Err(error) => {
            let body_response = BodyResponse { status: error };
            let mut response = Response::new(StatusCode::InternalServerError);
            response.set_body(Body::from_json(&body_response).unwrap());
            response
        }
    }
}
