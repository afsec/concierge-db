use brickpack::global_state::State;
use smol::block_on as smol_block_on;
use brickpack::{Request, Response, StatusCode};

use crate::api::Table;

use super::model;
use super::view;

pub fn handler(option_req: Option<Request<State>>) -> Response {
    let mut request = match option_req {
        Some(req) => req,
        None => {
            brickpack::log::error!("Request is None");
            return Response::new(StatusCode::InternalServerError);
        }
    };
    smol_block_on(async {
        // Get Table from json body
        let table_struct: Table = match request.body_json().await {
            Ok(table_name) => table_name,
            Err(error) => {
                brickpack::log::error!("{}", error);
                Table::default()
            }
        };

        let table_name = match table_struct.table {
            Some(value) => value,
            None => {
                let msg = "Table name not defined".to_string();
                brickpack::log::error!("{}", &msg);
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
        match model::show_columns(db_conn, table_name) {
            Ok(model) => {
                // View
                let view = view::show_columns(model);
                view
            }
            Err(error) => {
                let msg = format!("model::show_columns -> Err({})", error);
                brickpack::log::error!("{}", msg);
                Response::new(StatusCode::InternalServerError)
            }
        }
    })
}
