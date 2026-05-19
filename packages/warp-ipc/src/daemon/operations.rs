use serde::{Deserialize, Serialize};

use crate::daemon::types::{FamiliesMode, OperationMode};

/// Operation variants for IPC communication with the Cloudflare Warp service.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SuppressBroadcast {
    mode: bool,
}

/// Module defining the operations that can be performed via IPC communication with the Cloudflare Warp service.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged, rename_all = "snake_case")]
pub enum Operation {
    // Get Operations
    GetAppSettings,
    GetRegistrationInfo,
    GetDaemonStatus,
    GetNetworkInfo,
    GetMDMConfigurations,
    GetSupportUrl,
    GetAllowModeSwitch,
    GetOverrideEnd,
    GetPauseEnd,
    GetSecondsUntilLocalNetworkAccessEnd,
    GetIncludedHosts,
    GetIncludeRoutes,
    GetExcludedIps,
    GetExcludedHosts,
    GetVirtualNetworks,
    GetWarpStats,

    // Set Operations
    SetAlwaysOn {
        set_always_on: bool,
    },
    SetSuppressBroadcast {
        set_suppress_broadcast: SuppressBroadcast,
    },
    SetMode {
        set_mode: OperationMode,
    },
    SetDisableWiFi {
        set_disable_wi_fi: bool,
    },
    SetDisableEthernet {
        set_disable_ethernet: bool,
    },
    SetFamiliesMode {
        set_families_mode: FamiliesMode,
    },
    SetLicense {
        license: Vec<u8>, // The license string converted to an array of ascii codes
    },

    // Delete Operations
    DeleteRegistration,

    // Other Operations
    NewRegistration,
}
