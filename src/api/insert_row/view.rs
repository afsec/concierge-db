use tide::{Response, StatusCode};

use crate::api::{BodyResponse, StatusMessage};

pub fn insert_row(model_result: Result<StatusMessage, StatusMessage>) -> Response {
    match model_result {
        Ok(message) => {
            let body_response = BodyResponse { status: message };
            Response::new(StatusCode::Ok)
                .body_json(&body_response)
                .unwrap()
        }
        Err(error) => {
            let body_response = BodyResponse { status: error };
            Response::new(StatusCode::InternalServerError)
                .body_json(&body_response)
                .unwrap()
        }
    }
}
