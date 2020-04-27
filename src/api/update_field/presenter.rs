use tide::{Request, Response};

use super::model;
use super::view;
use crate::api::Coluna;

pub async fn handler(mut request: Request<()>) -> tide::Result {
    use crate::auth::is_authenticated;
    use http_types::StatusCode;

    // Authentication:
    if is_authenticated(&request) {
        // Get url param
        let table: String = match request.param("table") {
            Ok(table) => table,
            Err(error) => {
                return Err(http_types::Error::from_str(
                    StatusCode::BadRequest,
                    format!("Invalid parameter -> Err({})", error),
                ))
            }
        };

        // Get body content
        let colunas: Vec<Coluna> = match request.body_json().await {
            Ok(data) => data,
            Err(error) => {
                return Err(http_types::Error::from_str(
                    StatusCode::BadRequest,
                    format!("Invalid body -> Err({})", error),
                ))
            }
        };
        // Model
        match model::update_field(table, colunas) {
            Ok(model) => {
                // View
                let view = view::update_field(model);
                Ok(view)
            }
            Err(error) => Ok(Response::new(StatusCode::Ok).body_json(&error).unwrap()),
        }
    } else {
        Err(http_types::Error::from_str(
            StatusCode::Unauthorized,
            "Access Denied",
        ))
    }
}
