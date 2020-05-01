use tide::Request;

use crate::web_server::State;

use super::model;
use super::view;

pub async fn handler(request: Request<State>) -> tide::Result {
    use crate::auth::is_authenticated;
    use http_types::StatusCode;
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
}
