#![warn(clippy::all)]

mod api;
mod database;

use brickpack::App;
use clap::{crate_authors, crate_description, crate_name, crate_version, App as ClapApp};
use std::process::exit;

use crate::database::{bootstrap, connection};

fn main() {
    ClapApp::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .get_matches();

    let addr = "0.0.0.0";
    let port = "3341";

    let listen = format!("{}:{}", addr, port);

    let mut app = App::new();

    app.set_name(crate_name!());
    app.set_version(crate_version!());

    let db_conn = match connection() {
        Ok(conn) => conn,
        Err(error) => {
            brickpack::log::error!("{}", error);
            exit(1);
        }
    };

    bootstrap(&db_conn);

    app.add_endpoint("count-rows", crate::api::read_count::presenter::handler);
    app.add_endpoint("show-tables", crate::api::show_tables::presenter::handler);
    app.add_endpoint("read-all", crate::api::read_all::presenter::handler);
    app.add_endpoint("show-columns", crate::api::show_columns::presenter::handler);
    app.add_endpoint("insert-row", crate::api::insert_row::presenter::handler);
    app.add_endpoint("update-field", crate::api::update_field::presenter::handler);

    app.set_db_connection(db_conn);

    app.set_listen(listen);

    app.run().unwrap();
}
