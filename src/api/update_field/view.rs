use crate::api::{Response, StatusMessage};

pub fn update_field(message: StatusMessage) -> String {
    let response = Response { status: message };

    match serde_json::to_string::<Response>(&response) {
        Ok(res) => res,
        Err(err) => err.to_string(),
    }
}
