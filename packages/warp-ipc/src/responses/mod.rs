pub mod monitoring;
pub mod network;
pub mod settings;
pub mod mdm;

/// Used in `set_always_on` and `set_suppress_broadcast` responses (probably used in other `set_` responses as well)
pub struct SetResponse {
    pub ipc_status: String, // "success" or "error"
}
