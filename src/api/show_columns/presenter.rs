// use crate::auth::{is_valid_token, AuthorizationBearer};
use rocket::response::{content, status};
use rocket::get;

use super::model;
use super::view;

#[get("/api/<table>/show-columns", format = "json")]
pub fn show_columns(
    // bearer: &AuthorizationBearer,
    table: String
) -> Result<content::Json<String>, status::BadRequest<String>> {
    // if is_valid_token(bearer) {
        match model::show_columns(table) {
            Ok(model) => {
                dbg!(&model);
                let view = view::show_columns(model);
                Ok(content::Json(view))
            }
            Err(erro) => Err(status::BadRequest(Some(erro.to_string()))),
        }
    // } else {
    //     Err(status::BadRequest(Some(String::from(
    //         "{ \"status\": \"UnknownClient\"}",
    //     ))))
    // }
}
