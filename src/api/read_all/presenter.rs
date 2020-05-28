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
            let db_connection = match request.state().db_conn.get() {
                Ok(conn) => conn,
                Err(error) => {
                    let error_string = format!("Database connection error: {}", error.to_string());
                    log::error!("{}", &error_string);
                    return Err(http_types::Error::from_str(
                        StatusCode::InternalServerError,
                        error_string,
                    ));
                }
            };
            match request.param("table") {
                Ok(table) => match model::read_all(db_connection, table) {
                    Ok(model) => {
                        let view = view::read_all(model);
                        Ok(view)
                    }
                    Err(error) => {
                        let error_string = format!("model::show_tables -> Err({})", error);
                        log::error!("{}", &error_string);
                        Err(http_types::Error::from_str(
                            StatusCode::InternalServerError,
                            error_string,
                        ))
                    }
                },
                Err(error) => {
                    let error_string = format!("Invalid parameter -> Err({})", error);
                    log::error!("{}", &error_string);
                    Err(http_types::Error::from_str(
                        StatusCode::BadRequest,
                        error_string,
                    ))
                }
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
