#![warn(clippy::all)]

mod api;
mod auth;
mod database;
mod in_memory_db;
mod maintenance;
mod web_server;

fn main() {
    use clap::{crate_authors, crate_description, crate_name, crate_version, App};

    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .get_matches();

    let listen_addr = "0.0.0.0";
    let listen_port = "3341";
    let bind = format!("{}:{}", listen_addr, listen_port);

    println!("{} v{}", crate_name!(), crate_version!());

    // Run Tide server
    web_server::run(bind).unwrap();
}
