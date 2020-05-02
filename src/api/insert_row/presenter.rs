use http_types::StatusCode;
use tide::{Request, Response};

use crate::api::Coluna;
use crate::api::{BodyResponse, StatusMessage};
use crate::auth::{is_authenticated, is_in_maintenance_mode};
use crate::in_memory_db::State;

use super::model;
use super::view;

pub async fn handler(mut request: Request<State>) -> tide::Result {
    // Check Maintenance Mode
    if !is_in_maintenance_mode(&request) {
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
            match model::insert_row(db_connection, table, colunas) {
                Ok(model) => {
                    // View
                    let view = view::insert_row(model);
                    Ok(view)
                }
                Err(error) => Ok(Response::new(StatusCode::InternalServerError)
                    .body_json(&error)
                    .unwrap()),
            }
        } else {
            Err(http_types::Error::from_str(
                StatusCode::Unauthorized,
                "Access Denied",
            ))
        }
    } else {
        let body_response = BodyResponse {
            status: StatusMessage::UnderMaintenance,
        };
        Ok(Response::new(StatusCode::ServiceUnavailable)
            .body_json(&body_response)
            .unwrap())
    }
}
