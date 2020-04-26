use tide::Request;

use super::model;
use super::view;

pub async fn handler(request: Request<()>) -> tide::Result {
    use crate::auth::is_authenticated;
    use http_types::StatusCode;
    // Authorization:
    if is_authenticated(&request) {
        match request.param("table") {
            Ok(table) => match model::read_count(table) {
                Ok(model) => {
                    let view = view::read_count(model);
                    Ok(view)
                }
                Err(error) => Err(http_types::Error::from_str(
                    StatusCode::InternalServerError,
                    format!("model::show_tables -> Err({})", error),
                )),
            },
            Err(error) => Err(http_types::Error::from_str(
                StatusCode::BadRequest,
                format!("Invalid parameter -> Err({})", error),
            )),
        }
    } else {
        Err(http_types::Error::from_str(
            StatusCode::Unauthorized,
            "Access Denied",
        ))
    }
}
