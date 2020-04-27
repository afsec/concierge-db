use tide::Request;

use super::model;
use super::view;

pub async fn handler(request: Request<()>) -> tide::Result {
    use crate::auth::is_authenticated;
    use http_types::StatusCode;
    // Authentication:
    if is_authenticated(&request) {
        match model::show_tables() {
            Ok(model) => {
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
