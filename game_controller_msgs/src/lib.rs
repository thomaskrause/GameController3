//! This crate defines the messages that the GameController and associated tools exchange via the
//! network and their binary representations. The structs are deliberately not generated by
//! bindgen, but they are still rather raw and do not use types from [mod@game_controller::types].

mod bindings;
mod control_message;
mod monitor_request;
mod status_message;

use bindings::{
    GAMECONTROLLER_DATA_PORT, GAMECONTROLLER_RETURN_PORT, GAMECONTROLLER_RETURN_STRUCT_SIZE,
    GAMECONTROLLER_STRUCT_SIZE,
};

/// The binary size of a control message.
pub const CONTROL_MESSAGE_SIZE: usize = GAMECONTROLLER_STRUCT_SIZE;
/// The binary size of a monitor request.
pub const MONITOR_REQUEST_SIZE: usize = 5;
/// The binary size of a status message.
pub const STATUS_MESSAGE_SIZE: usize = GAMECONTROLLER_RETURN_STRUCT_SIZE;
/// The maximal binary size of a team message.
pub const TEAM_MESSAGE_MAX_SIZE: usize = 128;

/// The UDP port on which control messages are sent.
pub const CONTROL_MESSAGE_PORT: u16 = GAMECONTROLLER_DATA_PORT;
/// The UDP port on which monitor requests are received.
pub const MONITOR_REQUEST_PORT: u16 = 3636;
/// The UDP port on which status messages are received.
pub const STATUS_MESSAGE_PORT: u16 = GAMECONTROLLER_RETURN_PORT;
/// The UDP port on which status messages are forwarded.
pub const STATUS_MESSAGE_FORWARD_PORT: u16 = STATUS_MESSAGE_PORT + 1;
/// The number to which the team number is added to obtain the UDP port for that team's
/// communication.
pub const TEAM_MESSAGE_PORT_BASE: u16 = 10000;

pub use control_message::ControlMessage;
pub use monitor_request::MonitorRequest;
pub use status_message::StatusMessage;