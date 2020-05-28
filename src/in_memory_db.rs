use crate::database::Pool;
use std::sync::RwLock;

#[derive(Debug)]
pub struct State {
    pub db_conn: Pool,
    pub maintenance_mode: RwLock<bool>,
}

impl State {
    pub fn new(db_conn: Pool) -> Self {
        State {
            db_conn,
            maintenance_mode: RwLock::new(false),
        }
    }
}
