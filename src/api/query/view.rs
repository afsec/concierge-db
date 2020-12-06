use super::{QueryDatabase, InternalMessage};

use brickpack::endpoint::{Name, View};
use serde_json::to_string as serde_json_to_string;
use tide::{http::mime, Error as TideError, Response, StatusCode};

impl View<InternalMessage> for QueryDatabase {
    fn view(&self, result: Result<InternalMessage, TideError>) -> Response {
        let mut response = Response::builder(StatusCode::Ok)
            .content_type(mime::JSON)
            .build();
        match result {
            Ok(outcome) => {
                let json_body = serde_json_to_string(&outcome).unwrap_or("".to_owned());
                response.set_body(json_body);
            }
            Err(error) => {
                tide::log::error!(r#"Endpoint [{}]: {}"#, self.name(), error);
                #[cfg(debug_assertions)]
                response.insert_header("internal-error", error.to_string());

                response.set_body("[]");
            }
        }
        response
    }
}
