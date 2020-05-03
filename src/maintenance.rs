use crate::in_memory_db::State;

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