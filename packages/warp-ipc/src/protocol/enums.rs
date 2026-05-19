use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged, rename_all = "snake_case")]
pub enum RequestPayload {
    GetAppSettings,
    GetRegistrationInfo,
    SetAlwaysOn { set_always_on: bool },
}
