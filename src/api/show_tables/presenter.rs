use brickpack::global_state::State;
use brickpack::{Request, Response, StatusCode};

pub fn handler(option_req: Option<Request<State>>) -> Response {
    let request = match option_req {
        Some(req) => req,
        None => {
            brickpack::log::error!("Request is None");
            return Response::new(StatusCode::InternalServerError);
        }
    };

    // Get SqlitePooledConnecton
    let db_conn = match request.state().brickpack.get_db_connection() {
        Some(pool) => match pool.get() {
            Ok(conn) => conn,
            Err(error) => {
                brickpack::log::error!("{}", error);
                return Response::new(StatusCode::InternalServerError);
            }
        },
        None => {
            brickpack::log::error!("Cannot get PooledConnection");
            return Response::new(StatusCode::InternalServerError);
        }
    };

    // Model
    match super::model::show_tables(db_conn) {
        Ok(model) => {
            // View
            super::view::show_tables(model)
        }
        Err(error) => {
            let msg = format!("model::show_tables -> Err({})", error);
            brickpack::log::error!("{}", msg);
            Response::new(StatusCode::InternalServerError)
        }
    }
}
