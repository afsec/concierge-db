// use crate::auth::{is_valid_token, AuthorizationBearer};
use rocket::response::{content, status};
use rocket::get;

use super::model;
use super::view;

#[get("/api/<table>/count-rows", format = "json")]
pub fn read_count(
    // bearer: &AuthorizationBearer,
    table: String
) -> Result<content::Json<String>, status::BadRequest<String>> {
    // if is_valid_token(bearer) {
        match model::read_count(table) {
            Ok(model) => {
                let view = view::read_count(model);
                Ok(content::Json(view))
            }
            Err(erro) => Err(status::BadRequest(Some(erro))),
        }
    // } else {
    //     Err(status::BadRequest(Some(String::from(
    //         "{ \"status\": \"UnknownClient\"}",
    //     ))))
    // }
}
