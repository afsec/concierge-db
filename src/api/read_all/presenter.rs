// use crate::auth::{is_valid_token, AuthorizationBearer};
use crate::auth::UserAgent;
use rocket::response::{content, status};
use rocket::get;

use super::model;
use super::view;



#[get("/api/<table>/read-all", format = "json")]
pub fn read_all(
    ua: &UserAgent,
    // bearer: &AuthorizationBearer,
    table: String
) -> Result<content::Json<String>, status::BadRequest<String>> {
    // if is_valid_token(bearer) {
        println!("User-Agent: {}", ua.0.as_ref().unwrap());
        match model::read_all(table) {
            Ok(model) => {
                let view = view::read_all(model);
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
