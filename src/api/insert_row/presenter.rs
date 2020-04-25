use rocket::post;

use rocket::response::{content, status};
use rocket::Data;

use super::{model, view};
use crate::api::converters;

#[post("/api/<tablename>/insert-one", format = "json", data = "<data>")]
pub fn insert_row(
    tablename: String,
    data: Data,
) -> Result<content::Json<String>, status::BadRequest<String>> {
    let data_str = converters::data_to_string(data);
    match model::insert_row(tablename, data_str) {
        Ok(status) => {
            let view = view::insert_row(status);
            Ok(content::Json(view))
        }
        Err(status) => {
            let view = view::insert_row(status);
            Err(status::BadRequest(Some(view)))
        }
    }
    // let view = view::insert_row(&model);
    // Ok(content::Json(view))
}
