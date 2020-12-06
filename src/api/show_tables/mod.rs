mod model;
mod view;

use brickpack::{
    build_presenter,
    endpoint::{Endpoint, Name, Outcome, Presenter},
};
use brickpack_derive::{Endpoint, Outcome};
use serde::Serialize;

#[derive(Debug, Outcome, Serialize)]
struct InternalMessage(Vec<String>);

#[derive(Debug, Endpoint)]
#[endpoint_name = "show_tables"]
struct ShowTables;

build_presenter!(ShowTables, InternalMessage);
