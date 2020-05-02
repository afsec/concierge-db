use crate::database::Pool;
use std::sync::RwLock;

#[derive(Debug)]
pub struct State {
    pub db_conn: Pool,
    maintenance_mode: RwLock<bool>,
}

impl State {
    pub fn new(db_conn: Pool) -> Self {
        State {
            db_conn,
            maintenance_mode: RwLock::new(false),
        }
    }
}

pub struct MaintenanceMode;

impl MaintenanceMode {
    pub fn set_on(context: &State) {
        let mut maintenance_mode = context.maintenance_mode.write().unwrap();
        *maintenance_mode = true;
    }

    pub fn set_off(context: &State) {
        let mut maintenance_mode = context.maintenance_mode.write().unwrap();
        *maintenance_mode = false;
    }

    pub fn get_mode(context: &State) -> bool {
        let maintenance_mode = context.maintenance_mode.read().unwrap();
        *maintenance_mode
    }
}
