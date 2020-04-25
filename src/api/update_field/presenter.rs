use rocket::patch;

use rocket::response::{content, status};
use rocket::Data;

use super::{model, view};
use crate::api::converters;

#[patch("/api/<tablename>/update-field", format = "json", data = "<data>")]
pub fn update_field(
    tablename: String,
    data: Data,
) -> Result<content::Json<String>, status::BadRequest<String>> {
    let data_str = converters::data_to_string(data);
    match model::update_field(tablename, data_str) {
        Ok(status) => {
            let view = view::update_field(status);
            Ok(content::Json(view))
        }
        Err(status) => {
            let view = view::update_field(status);
            Err(status::BadRequest(Some(view)))
        }
    }
    // let view = view::update_field(&model);
    // Ok(content::Json(view))
}
