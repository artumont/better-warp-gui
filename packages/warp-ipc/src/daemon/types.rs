use serde::{Deserialize, Serialize};

// Warp modes used in `set_mode` operation, determines the type of connection used by the Warp service.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged, rename_all = "camelCase")]
pub enum OperationMode {
    Warp,
    DnsOverTls,
    DnsOverHttps,
    WarpWithDnsOverHttps,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged, rename_all = "camelCase")]
pub enum FamiliesMode {
    Off,
    Malware,
    Full,
}
