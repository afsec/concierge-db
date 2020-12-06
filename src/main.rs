#![warn(clippy::all)]

mod api;

use brickpack::app::App;
use brickpack_derive::App;

use std::mem;

use clap::{crate_authors, crate_description, App as ClapApp};
use tide::{Request, Response, StatusCode};

#[derive(App)]
struct MyApp;

#[async_std::main]
async fn main() -> tide::Result<()> {
    ClapApp::new(MyApp.name())
        .version(MyApp.version())
        .author(crate_authors!())
        .about(crate_description!())
        .get_matches();

    // Database bootstrap
    db_bootstrap();

    let addr = "127.0.0.1";
    let port = "3341";

    let listen = format!("{}:{}", addr, port);
    assert_eq!(0, mem::size_of_val(&MyApp));
    tide::log::start();
    let mut app = tide::new();
    tide::log::info!("Starting App [{} v{}]:", MyApp.name(), MyApp.version());
    tide::log::info!(
        "Powered by {} v{}",
        MyApp.powered_desc(),
        MyApp.powered_ver()
    );
    app.at("/").get(index_page);
    app.at("/auth").get(check_auth);
    app.at("/maintenance").patch(maintenance_mode);
    app.at("/api/:endpoint").post(dispatcher);
    app.listen(listen).await?;
    Ok(())
}

fn db_bootstrap() {
    use rusqlite::Connection;
    match Connection::open("database.sqlite3") {
        Ok(conn) => {
            let result = conn.execute_batch(
                r#"
                CREATE TABLE "users" (
                    "id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
                    "name"	TEXT NOT NULL
                );
                CREATE TABLE "groups" (
                    "id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
                    "name"	TEXT NOT NULL
                );
                INSERT into `users` (name) VALUES ("Charlie Root");
                INSERT into `users` (name) VALUES ("Laurem Ipsum");
                INSERT into `users` (name) VALUES ("Bozo");

                INSERT into `groups` (name) VALUES ("admin");
                INSERT into `groups` (name) VALUES ("staff");
                INSERT into `groups` (name) VALUES ("public");

            "#,
            );
            if let Err(error) = result {
                tide::log::error!("System [db_bootstrap]: {}", error.to_string());
            }
        }
        Err(error) => {
            tide::log::error!("System [db_bootstrap]: {}", error.to_string());
        }
    }
}

async fn index_page(req: Request<()>) -> tide::Result {
    drop(req);
    // TODO
    // Ok(Response::new(StatusCode::ServiceUnavailable)) // If is in Maintenance Mode
    Ok(Response::new(StatusCode::Found))
}

async fn check_auth(req: Request<()>) -> tide::Result {
    drop(req);
    // TODO
    // Ok(Response::new(StatusCode::Unauthorized)) // If auth is invalid
    Ok(Response::new(StatusCode::Accepted))
}

async fn maintenance_mode(req: Request<()>) -> tide::Result {
    drop(req);
    // TODO
    // Ok(Response::new(StatusCode::NotAcceptable))
    Ok(Response::new(StatusCode::Accepted))
}

async fn dispatcher(mut request: Request<()>) -> tide::Result {
    let endpoint = request.param("endpoint")?;

    match endpoint {
        "show_tables" => crate::api::show_tables::handler(request.body_string().await?),
        "query" => crate::api::query::handler(request.body_string().await?),
        _ => {
            tide::log::warn!("Not found: {}", endpoint);
            Ok(Response::new(StatusCode::NotFound))
        }
    }
}
