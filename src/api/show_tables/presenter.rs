use tide::{Request, Response};

// use super::model;
// use super::view;
use crate::api::{BodyResponse, StatusMessage};

pub async fn show_tables(request: Request<()>) -> tide::Result {
    use crate::auth::is_authenticated;
    use http_types::StatusCode;
    // Authorization:
    if is_authenticated(&request) {
        let body_response = BodyResponse {
            status: StatusMessage::NotImplemented,
        };
        Ok(Response::new(StatusCode::Ok)
            .body_json(&body_response)
            .unwrap())
    } else {
        Err(http_types::Error::from_str(
            StatusCode::Unauthorized,
            "Access Denied",
        ))
    }
}

// #[get("/api/show-tables", format = "json")]
// pub fn show_tables(
//     bearer: &AuthorizationBearer,
// ) -> Result<content::Json<String>, status::BadRequest<String>> {
//     if is_valid_token(bearer) {
//         match model::show_tables() {
//             Ok(model) => {
//                 dbg!(&model);
//                 let view = view::show_tables(model);
//                 Ok(content::Json(view))
//             }
//             Err(erro) => Err(status::BadRequest(Some(erro.to_string()))),
//         }
//     } else {
//         Err(status::BadRequest(Some(String::from(
//             "{ \"status\": \"UnknownClient\"}",
//         ))))
//     }
// }
