use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::daemon::operations::Operation;

/// Request structure for IPC communication with the Cloudflare Warp service.
#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub id: Uuid,
    pub request: Operation,
}

/// Response structure for IPC communication with the Cloudflare Warp service.
#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub id: Uuid,
    pub response: serde_json::Value,
}
