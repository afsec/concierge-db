use tide::{Request, Response};

pub fn run(bind: String) -> Result<(), std::io::Error> {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    const PROGRAM_NAME: &str = env!("CARGO_PKG_NAME");

    use async_std::task;
    use tide::Server;

    task::block_on(async {
        // let mut app = Server::with_state();
        let mut app = Server::new();
        app.at("/").get(main_index);
        // app.at("/auth").get(check_auth);
        app.at("/api/show-tables")
            .get(crate::api::show_tables::presenter::show_tables);
        // app.at("/auth")
        //     .post(crate::api::show_columns::presenter::show_columns);
        // app.at("/auth")
        //     .post(crate::api::read_count::presenter::read_count);
        // app.at("/auth")
        //     .post(crate::api::read_all::presenter::read_all);
        // app.at("/auth")
        //     .post(crate::api::insert_row::presenter::insert_row);
        // app.at("/auth")
        //     .post(crate::api::update_field::presenter::update_field);

        println!("{} v{}", PROGRAM_NAME, VERSION);
        println!("Listening at: http://{}", bind);

        app.listen(bind).await?;
        std::process::exit(0);
    })
}



pub async fn main_index(request: Request<()>) -> tide::Result {
    use crate::auth::is_authenticated;
    use http_types::{headers::CONTENT_TYPE, StatusCode};
    use mime::TEXT_HTML_UTF_8;
    // Authorization:
    if is_authenticated(&request) {
        let response = Response::new(StatusCode::Ok)
            .body_string(include_str!("./../html/home.html").to_string())
            .set_header(CONTENT_TYPE, TEXT_HTML_UTF_8);
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
