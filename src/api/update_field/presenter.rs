use tide::{Request, Response};

use crate::api::Coluna;
use crate::web_server::State;

use super::model;
use super::view;

pub async fn handler(mut request: Request<State>) -> tide::Result {
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

        // Get database connection
        let db_connection = match request.state().db_conn.get() {
            Ok(conn) => conn,
            Err(error) => {
                return Err(http_types::Error::from_str(
                    StatusCode::InternalServerError,
                    format!("Database connection error: {}", error.to_string()),
                ))
            }
        };

        // Model
        match model::update_field(db_connection, table, colunas) {
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
