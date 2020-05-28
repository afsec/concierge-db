#![warn(clippy::all)]

mod api;
mod auth;
mod database;
mod in_memory_db;
mod maintenance;
mod web_server;

use auth::SERVER_TOKEN_ENV_VAR;

fn main() -> Result<(), std::io::Error> {
    use clap::{crate_authors, crate_description, crate_name, crate_version, App};

    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .get_matches();

    // * Start log runtime: femme
    femme::start();

    let listen_addr = "0.0.0.0";
    let listen_port = "3341";
    let bind = format!("{}:{}", listen_addr, listen_port);

    println!("{} v{}", crate_name!(), crate_version!());
    match crate::auth::get_token_from_env(SERVER_TOKEN_ENV_VAR) {
        Some(token) => {
            println!();
            println!("{}: {}", SERVER_TOKEN_ENV_VAR, token);
            println!();
            web_server::run(bind).unwrap();
            std::process::exit(0);
        }
        None => {
            log::error!(
                "Environment variable {} not found",
                SERVER_TOKEN_ENV_VAR
            );
            std::process::exit(1);
        }
    }
}
