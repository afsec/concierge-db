use brickpack::global_state::State;
use tide::{Request, Response, StatusCode};
use smol::block_on as smol_block_on;

use crate::api::Table;

use super::model;
use super::view;

pub fn handler(option_req: Option<Request<State>>) -> Response {
    let mut request = match option_req {
        Some(req) => req,
        None => {
            tide::log::error!("Request is None");
            return Response::new(StatusCode::InternalServerError);
        }
    };
    smol_block_on(async {
        // Get Table from json body
        let table_struct: Table = match request.body_json().await {
            Ok(table_name) => table_name,
            Err(error) => {
                tide::log::error!("{}", error);
                Table::default()
            }
        };

        let table_name = match table_struct.table {
            Some(value) => value,
            None => {
                let msg = "Table name not defined".to_string();
                tide::log::error!("{}", &msg);
                let mut response = Response::new(StatusCode::BadRequest);
                response.set_body(msg);
                return response
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
    match model::read_all(db_conn, table_name) {
            Ok(model) => {
                let view = view::read_all(model);
                view
            }
            Err(error) => {
                let msg = format!("model::read_all -> Err({})", error);
                tide::log::error!("{}", msg);
                Response::new(StatusCode::InternalServerError)
            }
        }
    })
}
