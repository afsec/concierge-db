use tide::Request;

use crate::in_memory_db::State;
use crate::maintenance::MaintenanceMode;
use http_types::headers::AUTHORIZATION;
pub const SERVER_TOKEN_ENV_VAR: &str = "CONCIERGE_SERVER_TOKEN";

fn get_token(request: &Request<State>) -> Option<String> {
    match request.header(AUTHORIZATION) {
        Some(headers) => match headers.get(0) {
            Some(header_value) => {
                let header_value = header_value.to_string();
                match header_value.split_whitespace().last() {
                    Some(token) => Some(token.to_string()),
                    None => None,
                }
            }
            None => None,
        },
        None => None,
    }
}

fn is_valid_token(token_from_request: String) -> bool {
    // TODO: Add more info (Request IP Address)
    match get_token_from_env(SERVER_TOKEN_ENV_VAR) {
        Some(token) => {
            if token_from_request == token {
                log::debug!(
                    "Autenticated {}: {:?}",
                    SERVER_TOKEN_ENV_VAR,
                    token_from_request
                );
                true
            } else {
                log::error!(
                    "ACCESS DENIED {}: {:?}",
                    SERVER_TOKEN_ENV_VAR,
                    token_from_request
                );
                false
            }
        }
        None => {
            log::error!(
                "Environment variable {} not found",
                SERVER_TOKEN_ENV_VAR
            );
            false
        }
    }
}

pub fn get_token_from_env(token_name: &str) -> Option<String> {
    use std::env;
    match env::var(token_name) {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}

pub fn is_authenticated(request: &Request<State>) -> bool {
    match get_token(&request) {
        Some(token) => is_valid_token(token),
        None => false,
    }
}

pub fn is_in_maintenance_mode(request: &Request<State>) -> bool {
    MaintenanceMode::get_mode(&request.state())
}
