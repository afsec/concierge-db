use http_types::StatusCode;
use tide::{Request, Response};

use crate::api::{BodyResponse, StatusMessage};
use crate::auth::{is_authenticated, is_in_maintenance_mode};
use crate::in_memory_db::State;

use super::model;
use super::view;

pub async fn handler(request: Request<State>) -> tide::Result {
    // Check Maintenance Mode
    if !is_in_maintenance_mode(&request) {
        // Authentication:
        if is_authenticated(&request) {
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
            match model::show_tables(db_connection) {
                Ok(model) => {
                    // View
                    let view = view::show_tables(model);
                    Ok(view)
                }
                Err(error) => Err(http_types::Error::from_str(
                    StatusCode::InternalServerError,
                    format!("model::show_tables -> Err({})", error),
                )),
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
