use tide::{Request, Response, StatusCode};

use super::model;
use super::view;
use brickpack::global_state::State;

pub fn handler(option_req: Option<Request<State>>) -> Response {
    let request = match option_req {
        Some(req) => req,
        None => {
            tide::log::error!("Request is None");
            return Response::new(StatusCode::InternalServerError);
        }
    };

    // Get SqlitePooledConnecton
    let db_conn = match request.state().brickpack.get_db_connection() {
        Some(pool) => match pool.get() {
            Ok(conn) => conn,
            Err(error) => {
                tide::log::error!("{}", error);
                return Response::new(StatusCode::InternalServerError);
            }
        },
        None => {
            tide::log::error!("Cannot get PooledConnection");
            return Response::new(StatusCode::InternalServerError);
        }
    };

        // Model
        match model::show_tables(db_conn) {
            Ok(model) => {
                // View
                let view = view::show_tables(model);
                view
            }
            Err(error) => {
                let msg = format!("model::show_tables -> Err({})", error);
                tide::log::error!("{}", msg);
                Response::new(StatusCode::InternalServerError)
            }
        }

}
