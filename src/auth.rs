use tide::Request;
use crate::web_server::State;

pub fn is_authenticated(request: &Request<State>) -> bool {
    match get_token(&request) {
        Some(token) => is_valid_token(token),
        None => false,
    }
}

fn get_token(request: &Request<State>) -> Option<String> {
    match request.header(&"Authorization".parse().unwrap()) {
        Some(vec) => match vec.first() {
            Some(header_value) => {
                let header_value = header_value.to_string();
                match header_value.split_whitespace().last() {
                    Some(token) => Some(token.to_string()),
                    None => None
                }
            },
            None => None,
        },
        None => None,
    }
}


pub fn is_valid_token(token: String) -> bool {
    // TODO: Implement Envvar -> `auth.toml`
    if token == "9admin9" {
        eprintln!("Authenticated: CLIENT_TOKEN: {:?}", token);
        true
    } else {
        eprintln!("ACCESS DENIED: CLIENT_TOKEN: {:?}", token);
        false
    }
}