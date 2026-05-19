use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetAppSettings {
    pub always_on: bool,
    pub switch_locked: bool,
    pub auto_connect: String,   // Unknown (got `null` on dump)
    pub operation_mode: String, // WarpWithDnsOverHttps
}

pub struct GetSupportUrl {}
