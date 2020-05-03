use tide::Request;

use crate::api::StatusMessage;
use crate::maintenance::MaintenanceMode;
use crate::in_memory_db::State;

pub fn maintenance_mode(
    request: &Request<State>,
    mode: bool,
) -> Result<StatusMessage, StatusMessage> {
    if mode {
        MaintenanceMode::set_on(&request.state());
    } else {
        MaintenanceMode::set_off(&request.state());
    }
    Ok(StatusMessage::Saved)
}
