use serde::{Deserialize, Serialize};

// Determines the type of connection used by the Warp service, such as "Warp" for a standard VPN connection, "DnsOverTls" for DNS over TLS, "DnsOverHttps" for DNS over HTTPS, and "WarpWithDnsOverHttps" for a combination of both Warp and DNS over HTTPS. This is used in the `set_mode` operation to specify the desired connection type.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged, rename_all = "camelCase")]
pub enum OperationMode {
    Warp,
    DnsOverTls,
    DnsOverHttps,
    WarpWithDnsOverHttps,
}

/// Determines the level of protection against malicious content provided by the Warp service. "Off" means no protection, "Malware" means blocking of known malicious domains, and "Full" means blocking of both malicious and adult content.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged, rename_all = "camelCase")]
pub enum FamiliesMode {
    Off,
    Malware,
    Full,
}
