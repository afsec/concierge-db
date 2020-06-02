use brickpack::global_state::State;
use smol::block_on as smol_block_on;
use tide::{Request, Response, StatusCode};

use crate::api::{Coluna, Table};

use super::model;
use super::view;

// * Presenter
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
        // // Get body content
        // let colunas: Vec<Coluna> = match request.body_json().await {
        //     Ok(data) => data,
        //     Err(error) => {
        //         tide::log::error!("Invalid Input");
        //         return Response::new(StatusCode::BadRequest);
        //     }
        // };

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

        let colunas: Vec<Coluna> = match table_struct.columns {
            Some(vec) => vec,
            None => {
                tide::log::error!("No Columns defined");
                Vec::new()
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

        // * Model
        let model = model::update_field(db_conn, table_name, colunas);
        // * View
        let view = view::update_field(model);
        view
    })
}
