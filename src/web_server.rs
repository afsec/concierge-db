use tide::{Request, Response};

use crate::in_memory_db::State;

// #[derive(Debug)]
// pub struct State {
//     pub db_conn: Pool,
// }

pub fn run(bind: String) -> Result<(), std::io::Error> {
    use crate::database;
    use async_std::task;
    use tide::Server;

    let conn = match database::connection() {
        Ok(conn) => conn,
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(1);
        }
    };

    database::bootstrap(&conn);

    task::block_on(async {
        let mut app = Server::with_state(State::new(conn));
        app.at("/").get(main_index);
        app.at("/auth").get(check_auth);
        app.at("/api/maintenance")
            .patch(crate::api::maintenance_mode::presenter::handler);
        app.at("/api/show-tables")
            .get(crate::api::show_tables::presenter::handler);
        app.at("/api/:table/read-all")
            .get(crate::api::read_all::presenter::handler);
        app.at("/api/:table/count-rows")
            .get(crate::api::read_count::presenter::handler);
        app.at("/api/:table/show-columns")
            .get(crate::api::show_columns::presenter::handler);
        app.at("/api/:table/insert-one")
            .post(crate::api::insert_row::presenter::handler);
        app.at("/api/:table/update-field")
            .patch(crate::api::update_field::presenter::handler);

        println!("Listening on: http://{}", bind);

        app.listen(bind).await?;
        std::process::exit(0);
    })
}

pub async fn main_index(request: Request<State>) -> tide::Result {
    use crate::auth::is_authenticated;
    use http_types::StatusCode;
    use mime::TEXT_HTML_UTF_8;

    // Authentication
    if is_authenticated(&request) {
        let response = Response::new(StatusCode::Ok)
            .body_string(include_str!("./../html/home.html").to_string())
            .set_mime(TEXT_HTML_UTF_8);
        Ok(response)
    } else {
        Err(http_types::Error::from_str(
            StatusCode::Unauthorized,
            "Access Denied",
        ))
    }
}

pub async fn check_auth(request: Request<State>) -> tide::Result {
    use crate::auth::is_authenticated;
    use http_types::StatusCode;
    if is_authenticated(&request) {
        let response = Response::new(StatusCode::Accepted);
        Ok(response)
    } else {
        Err(http_types::Error::from_str(
            StatusCode::Unauthorized,
            "Access Denied",
        ))
    }
}

// #[catch(404)]
// pub fn error_404() -> content::Html<&'static str> {
//     content::Html(include_str!("./../html/404.html"))
// }
