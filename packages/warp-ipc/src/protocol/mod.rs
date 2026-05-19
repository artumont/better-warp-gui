use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::protocol::enums::RequestPayload;

pub mod enums;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub id: Uuid,
    pub request: RequestPayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub id: Uuid,
    pub response: serde_json::Value,
}
